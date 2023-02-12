[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
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
                expectation_rule_idx: None,
                resolve_progress: ResolvedOk(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
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
                expectation_rule_idx: None,
                resolve_progress: ResolvedOk(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectationRules {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::ImplBlock(
                ImplBlockId {
                    module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
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
                expectation_rule_idx: None,
                resolve_progress: ResolvedOk(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectationRules {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        },
                    },
                    ident: `displacement`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
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
                expectation_rule_idx: None,
                resolve_progress: ResolvedOk(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectationRules {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: Some(
            Term(`LineSegment`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        },
                    },
                    ident: `dist_to_point`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
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
                expectation_rule_idx: None,
                resolve_progress: ResolvedOk(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            },
        ],
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`Point2d`),
            ),
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectationRules {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: Some(
            Term(`LineSegment`),
        ),
    },
]