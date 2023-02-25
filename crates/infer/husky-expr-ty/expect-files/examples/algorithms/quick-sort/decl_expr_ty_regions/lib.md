[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`quick_sort::quick_sort`, `Function`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
            ],
        },
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`Type`),
            ),
            LocalTerm::Resolved(
                Term(`Slice $0`),
            ),
        ],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
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
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`quick_sort::quick_sort_aux`, `Function`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        },
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`Type`),
            ),
            LocalTerm::Resolved(
                Term(`Slice $0`),
            ),
            LocalTerm::Resolved(
                Term(`core::num::isize`),
            ),
            LocalTerm::Resolved(
                Term(`core::num::isize`),
            ),
        ],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
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
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`quick_sort::partition`, `Function`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
                Some(
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
                            0,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
            ],
        },
        expr_local_terms: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`Type`),
            ),
            LocalTerm::Resolved(
                Term(`Slice $0`),
            ),
            LocalTerm::Resolved(
                Term(`core::num::isize`),
            ),
            LocalTerm::Resolved(
                Term(`core::num::isize`),
            ),
        ],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
                first_unresolved_term: 0,
            },
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 6,
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
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
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
            DeclRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [],
        },
        expr_local_terms: ArenaMap {
            data: [],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
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
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [],
        },
        expr_local_terms: ArenaMap {
            data: [],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [],
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