[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`natural_number_game::Nat`, `Inductive`),
            ),
        ),
        expr_ty_infos: [],
        expr_terms: ArenaMap {
            data: [],
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
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`natural_number_game::OddNat`, `Structure`),
            ),
        ),
        expr_ty_infos: [],
        expr_terms: ArenaMap {
            data: [],
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
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`natural_number_game::EvenNat`, `Structure`),
            ),
        ),
        expr_ty_infos: [],
        expr_terms: ArenaMap {
            data: [],
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
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::ImplBlock(
                ImplBlockId {
                    module_path: `natural_number_game`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`natural_number_game::Nat`, `Inductive`),
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
        expr_terms: ArenaMap {
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
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `natural_number_game`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`natural_number_game::Nat`, `Inductive`),
                        },
                    },
                    ident: `add`,
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
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        0,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: ResolvedOk(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            0,
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
                expectation_rule_idx: Some(
                    1,
                ),
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
                            Curry(
                                TermCurry(
                                    Id {
                                        value: 2,
                                    },
                                ),
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
                                Curry(
                                    TermCurry(
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
        ],
        expr_terms: ArenaMap {
            data: [
                None,
                Some(
                    Ok(
                        Resolved(
                            ReducedTerm(
                                Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 49,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                None,
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
                    data: [
                        LocalTermExpectationRule {
                            src_expr_idx: 2,
                            expectee: Resolved(
                                ReducedTerm(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                0,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                            variant: Type,
                            resolve_progress: ResolvedOk(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Resolved(
                                        ReducedTerm(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        0,
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 3,
                            expectee: Resolved(
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
                            variant: Type,
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
                },
                first_unresolved_expectation: 0,
            },
        },
    },
]