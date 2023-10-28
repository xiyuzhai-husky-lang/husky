[
    SemaExprRegion {
        [salsa id]: 280,
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Type(
                        TypeSynNodePath(
                            Id {
                                value: 54,
                            },
                        ),
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
                                PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 36,
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
                                Prefix {
                                    opr: Leash,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        3,
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
                                    path_expr_idx: 3,
                                    path: MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 57,
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
                                FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        4,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        5,
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
                    6,
                    SemaExprIdx(
                        6,
                    ),
                ),
            ],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 54,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 57,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [
                        None,
                        None,
                    ],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: CurryDestination(
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
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
                                                TermUniverse(
                                                    1,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: CurryDestination(
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
                                    idx: 3,
                                    src: ExpectationSource {
                                        expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: Sort,
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    template_parameter_substitutions: [],
                                                    return_ty: FluffyTerm {
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
                                                    variant: Curry {
                                                        variance: Covariant,
                                                        parameter_symbol: None,
                                                        parameter_ty: FluffyTerm {
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
                                                        return_ty: FluffyTerm {
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
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: CurryDestination(
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
                                    idx: 5,
                                    src: ExpectationSource {
                                        expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
    },
    SemaExprRegion {
        [salsa id]: 281,
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                ),
            ),
            sema_expr_arena: SemaExprArena(
                Arena {
                    data: [
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 2,
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
                                VecFunctor {
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
                                    path_expr_idx: 3,
                                    path: MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 59,
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
                                FunctionApplication {
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
            pattern_expr_ty_infos: ArenaMap {
                data: [
                    None,
                ],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [
                    None,
                ],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 54,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [
                        None,
                    ],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: CurryDestination(
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
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
                                                TermUniverse(
                                                    1,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: Sort,
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    template_parameter_substitutions: [],
                                                    return_ty: FluffyTerm {
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
                                                    variant: Curry {
                                                        variance: Covariant,
                                                        parameter_symbol: None,
                                                        parameter_ty: FluffyTerm {
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
                                                        return_ty: FluffyTerm {
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
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: CurryDestination(
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
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
    },
    SemaExprRegion {
        [salsa id]: 282,
        path: RegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Decl(
                ImplBlock(
                    TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNodePath {
                            path: TraitForTypeImplBlockPath(
                                Id {
                                    value: 30,
                                },
                            ),
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
                                                    value: 59,
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
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: AnyOriginal(
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
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 283,
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    trai_path: TraitPath(`core::visual::Visualize`),
                                    ty_sketch: TypeSketch::Path(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TraitForTypeItem(
                        TraitForTypeItemSynNodePath(
                            Id {
                                value: 21,
                            },
                        ),
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
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 284,
        path: RegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TypeImplBlock(
                    TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Decl(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockSynNodePath {
                            path: TypeImplBlockPath(
                                Id {
                                    value: 30,
                                },
                            ),
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
                                                    value: 59,
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
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 285,
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `norm`,
                                item_kind: MemoizedField,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            Id {
                                value: 85,
                            },
                        ),
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
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 286,
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `rel_norm`,
                                item_kind: MemoizedField,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            Id {
                                value: 86,
                            },
                        ),
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
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 287,
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `hausdorff_norm`,
                                item_kind: MemoizedField,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            Id {
                                value: 87,
                            },
                        ),
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
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 288,
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `angle_change`,
                                item_kind: MemoizedField,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            Id {
                                value: 88,
                            },
                        ),
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
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 289,
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            Id {
                                value: 89,
                            },
                        ),
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
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 290,
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            Id {
                                value: 90,
                            },
                        ),
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
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 291,
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `line_segment`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            Id {
                                value: 91,
                            },
                        ),
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
                                                    value: 61,
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
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 292,
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `start`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            Id {
                                value: 92,
                            },
                        ),
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
                    1,
                    SemaExprIdx(
                        1,
                    ),
                ),
            ],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 293,
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `end`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            Id {
                                value: 93,
                            },
                        ),
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
                    1,
                    SemaExprIdx(
                        1,
                    ),
                ),
            ],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 294,
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            Id {
                                value: 94,
                            },
                        ),
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
            ],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 295,
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `start_tangent`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            Id {
                                value: 95,
                            },
                        ),
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
            ],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        [salsa id]: 296,
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `end_tangent`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            Id {
                                value: 96,
                            },
                        ),
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
            ],
            pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_symbol_map: ArenaMap {
                    data: [],
                },
                current_symbol_map: ArenaMap {
                    data: [],
                },
            },
            fluffy_term_region: FluffyTermRegion {
                terms: FluffyTerms {
                    solid_terms: SolidTerms {
                        entries: VecSet {
                            data: [],
                        },
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
                                expectation: EqsSort(
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsSort(
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
                EntityPath(
                    TypeOntology(
                        TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                ),
            ),
        },
    },
]