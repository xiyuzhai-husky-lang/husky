[
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`covariant Type -> Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::Tilde(
                            TildeDisambiguation::Leash,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`covariant Type -> Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    4,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    5,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash CyclicSlice Point2d`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    6,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::MethodCallOrApplication(
                            MethodCallOrApplicationDisambiguation::MethodCall {
                                method_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyDynamicDispatchIndirections {
                                        indirections: [
                                            Leash,
                                        ],
                                    },
                                    signature: MethodFn(
                                        MethodFnFluffySignature {
                                            parenate_parameters: [],
                                            return_ty: FluffyTerm {
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
                                        },
                                    ),
                                },
                                ritchie_parameter_argument_matches: Ok(
                                    [],
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Option Leash Point2d`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    7,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::UnwrapOrComposeWithNot(
                            UnwrapOrComposeWithNotExprDisambiguation::Unwrap,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash Point2d`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    8,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::MethodCallOrApplication(
                            MethodCallOrApplicationDisambiguation::MethodCall {
                                method_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyDynamicDispatchIndirections {
                                        indirections: [
                                            Leash,
                                        ],
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
                                ritchie_parameter_argument_matches: Ok(
                                    [],
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Point2d`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    9,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    10,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash CyclicSlice Point2d`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    11,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::MethodCallOrApplication(
                            MethodCallOrApplicationDisambiguation::MethodCall {
                                method_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyDynamicDispatchIndirections {
                                        indirections: [
                                            Leash,
                                        ],
                                    },
                                    signature: MethodFn(
                                        MethodFnFluffySignature {
                                            parenate_parameters: [],
                                            return_ty: FluffyTerm {
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
                                        },
                                    ),
                                },
                                ritchie_parameter_argument_matches: Ok(
                                    [],
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Option Leash Point2d`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    12,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::UnwrapOrComposeWithNot(
                            UnwrapOrComposeWithNotExprDisambiguation::Unwrap,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash Point2d`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    13,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::MethodCallOrApplication(
                            MethodCallOrApplicationDisambiguation::MethodCall {
                                method_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyDynamicDispatchIndirections {
                                        indirections: [
                                            Leash,
                                        ],
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
                                ritchie_parameter_argument_matches: Ok(
                                    [],
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Point2d`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    14,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`Point2d`),
                    ),
                },
            ),
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`Point2d`),
                    ),
                },
            ),
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash CyclicSlice Point2d`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Point2d`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Point2d`),
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
                                        EtherealTerm(`covariant Type -> Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::Sort,
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
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
                                        EtherealTerm(`Leash CyclicSlice Point2d`),
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
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash Point2d`),
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
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash Point2d`),
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
                                            EtherealTerm(`Point2d`),
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
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
                                        EtherealTerm(`Leash CyclicSlice Point2d`),
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
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash Point2d`),
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
                                        EtherealTerm(`Leash Point2d`),
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
                                            EtherealTerm(`Point2d`),
                                        ),
                                    },
                                },
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::Tilde(
                            TildeDisambiguation::Leash,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::List(
                            ListExprDisambiguation::ListFunctor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`covariant Type -> Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    4,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    5,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash RawContour`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Vec LineSegmentStroke`),
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
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
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
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
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
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::Tilde(
                            TildeDisambiguation::Leash,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    4,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    5,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash RawContour`),
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
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::Tilde(
                            TildeDisambiguation::Leash,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    4,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    5,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    6,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash RawContour`),
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
                            EtherealTerm(`i32`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
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
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::Tilde(
                            TildeDisambiguation::Leash,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::List(
                            ListExprDisambiguation::ListFunctor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`covariant Type -> Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    4,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    5,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    6,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash RawContour`),
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
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::Sort,
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
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
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
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Trait`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
            EtherealTerm(`LineSegmentStroke`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch`,
                                    trai_path: TraitPath(`core::visual::Visualize`),
                                    ty_sketch: TypeSketch::Path(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
            EtherealTerm(`LineSegmentStroke`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TypeImplBlock(
                    TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
            EtherealTerm(`LineSegmentStroke`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `new`,
                                item_kind: AssociatedFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::Tilde(
                            TildeDisambiguation::Leash,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    4,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    5,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash RawContour`),
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
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
            EtherealTerm(`LineSegmentStroke`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Trait`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
            EtherealTerm(`LineSegmentSketch`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch`,
                                    trai_path: TraitPath(`core::visual::Visualize`),
                                    ty_sketch: TypeSketch::Path(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
            EtherealTerm(`LineSegmentSketch`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TypeImplBlock(
                    TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
            EtherealTerm(`LineSegmentSketch`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `concave_components`,
                                item_kind: MemoizedField,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::List(
                            ListExprDisambiguation::ListFunctor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`covariant Type -> Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: Some(
            EtherealTerm(`LineSegmentSketch`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
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
            EtherealTerm(`LineSegmentSketch`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `new`,
                                item_kind: AssociatedFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::Tilde(
                            TildeDisambiguation::Leash,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        SynExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Type`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    4,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash RawContour`),
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
]