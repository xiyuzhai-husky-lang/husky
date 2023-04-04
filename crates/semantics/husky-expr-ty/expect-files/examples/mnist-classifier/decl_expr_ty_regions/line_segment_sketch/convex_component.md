[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        TypePath(
                            Ontology,
                        ),
                        Ok(
                            Resolved(
                                Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: Expected(
                    Resolved(
                        Ok(
                            EqsExactly(
                                ExpectSubtypeOutcome,
                            ),
                        ),
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Resolved(
                                Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
                resolve_progress: Expected(
                    Resolved(
                        Ok(
                            EqsSort(
                                TermUniverse(
                                    1,
                                ),
                            ),
                        ),
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        List(
                            ArrayFunctor,
                        ),
                        Ok(
                            Resolved(
                                Curry(
                                    TermCurry(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
                resolve_progress: Expected(
                    Resolved(
                        Ok(
                            EqsRitchieCallType(
                                ExpectEqsFunctionTypeOutcome {
                                    implicit_parameter_substitutions: [],
                                    return_ty: Resolved(
                                        Category(
                                            TermCategory {
                                                universe: TermUniverse(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ),
                                    variant: Curry {
                                        parameter_symbol: None,
                                        parameter_ty: Resolved(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                        return_ty: Resolved(
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
                        ),
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        TypeError(
                            RawTypeError(
                                Derived(
                                    TypeOntologyDeclError {
                                        path: TypePath(
                                            Id {
                                                value: 82,
                                            },
                                        ),
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Resolved(
                                Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
                resolve_progress: Expected(
                    Resolved(
                        Ok(
                            EqsSort(
                                TermUniverse(
                                    1,
                                ),
                            ),
                        ),
                    ),
                ),
            },
        ],
        extra_expr_errors: [],
        expr_local_terms: [],
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        local_term_region: LocalTermRegion {
            unresolved_terms: UnresolvedTerms {
                implicit_symbol_registry: ImplicitSymbolRegistry {
                    next: 0,
                },
                data: [],
                first_unresolved_term: 0,
            },
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationRule {
                            src_expr_idx: 0,
                            expectee: Resolved(
                                Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                            ),
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: Resolved(
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
                            resolve_progress: Resolved(
                                Ok(
                                    EqsExactly(
                                        ExpectSubtypeOutcome,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 1,
                            expectee: Resolved(
                                Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                            ),
                            expectation: EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
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
                        LocalTermExpectationRule {
                            src_expr_idx: 4,
                            expectee: Resolved(
                                Curry(
                                    TermCurry(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                            expectation: EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: Sort,
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    EqsRitchieCallType(
                                        ExpectEqsFunctionTypeOutcome {
                                            implicit_parameter_substitutions: [],
                                            return_ty: Resolved(
                                                Category(
                                                    TermCategory {
                                                        universe: TermUniverse(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            variant: Curry {
                                                parameter_symbol: None,
                                                parameter_ty: Resolved(
                                                    Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                return_ty: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 6,
                            expectee: Resolved(
                                Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                            ),
                            expectation: EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
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
            DeclRegionPath::ImplBlock(
                ImplBlockId::Type(
                    TypeImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                        disambiguator: 0,
                    },
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        TypeError(
                            RawTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
        ],
        extra_expr_errors: [],
        expr_local_terms: [],
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        local_term_region: LocalTermRegion {
            unresolved_terms: UnresolvedTerms {
                implicit_symbol_registry: ImplicitSymbolRegistry {
                    next: 0,
                },
                data: [],
                first_unresolved_term: 0,
            },
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
]