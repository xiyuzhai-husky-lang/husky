[
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`quick_sort::quick_sort`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
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
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        TypeError(
                            DeclError,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CurrentSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BracketedItemTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
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
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            0,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Err(
                    Derived(
                        UnresolvedLocalTerm,
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CallableTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BlockTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 1,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 9,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                ],
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
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`quick_sort::quick_sort_aux`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        InheritedSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
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
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        InheritedSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        InheritedSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CallableTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CurrentSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
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
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        InheritedSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CallableTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CurrentSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
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
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        InheritedSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CallableTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BlockTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
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
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`quick_sort::partition`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        InheritedSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        InheritedSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        InheritedSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 1,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CurrentSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CurrentSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            1,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Err(
                    Derived(
                        UnresolvedLocalTerm,
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CurrentSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CurrentSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CurrentSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CurrentSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CurrentSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
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
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        TypeError(
                            DeclError,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
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
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        TypeError(
                            DeclError,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        CurrentSymbolTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BlockTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 1,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 17,
                                variant: ImplicitType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        unresolved_term: Application {
                            function: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 21,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            argument: Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                        },
                        unresolved_term_pattern: Injection {
                            function: ModuleItem(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 21,
                                        },
                                    ),
                                ),
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ],
                        },
                        resolve_progress: Err(
                            Original(
                                UnresolvedTerm,
                            ),
                        ),
                    },
                ],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectationRules {
                arena: Arena {
                    data: [
                        LocalTermExpectationRule {
                            src_expr_idx: 7,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            variant: AsBool,
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Resolved(
                                        ReducedTerm(
                                            Entity(
                                                ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 1,
                                                            },
                                                        ),
                                                    ),
                                                ),
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
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            4,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Err(
                    Derived(
                        UnresolvedLocalTerm,
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            1,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            2,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            3,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            4,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            5,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    4,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            6,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    5,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            7,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    6,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            8,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    7,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            9,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    8,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            10,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    9,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            11,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Err(
                    Derived(
                        UnresolvedLocalTerm,
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            11,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Err(
                    Derived(
                        UnresolvedLocalTerm,
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        FunctionTypeNotInferredInApplicationOrFunctionCall,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            11,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Err(
                    Derived(
                        UnresolvedLocalTerm,
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 22,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 22,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 11,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 11,
                                variant: ImplicitType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 1,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 2,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 3,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
                                ),
                                src_expr_idx: 0,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    5,
                                ),
                                src_expr_idx: 5,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    6,
                                ),
                                src_expr_idx: 6,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    7,
                                ),
                                src_expr_idx: 7,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    8,
                                ),
                                src_expr_idx: 8,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    9,
                                ),
                                src_expr_idx: 9,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    10,
                                ),
                                src_expr_idx: 10,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                0,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: Application {
                            function: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 21,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            argument: Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                        },
                        unresolved_term_pattern: Injection {
                            function: ModuleItem(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 21,
                                        },
                                    ),
                                ),
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ],
                        },
                        resolve_progress: Err(
                            Original(
                                UnresolvedTerm,
                            ),
                        ),
                    },
                ],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectationRules {
                arena: Arena {
                    data: [
                        LocalTermExpectationRule {
                            src_expr_idx: 1,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 2,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    2,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 3,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    3,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 4,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    4,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 5,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    5,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 6,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    6,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 7,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    7,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 8,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    8,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 9,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    9,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 10,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    10,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            0,
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
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 3,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
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
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 3,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
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
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 3,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
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
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 3,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
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
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 3,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    4,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
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
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 3,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    5,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
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
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            1,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Err(
                    Derived(
                        UnresolvedLocalTerm,
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            1,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Err(
                    Derived(
                        UnresolvedLocalTerm,
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        FunctionTypeNotInferredInApplicationOrFunctionCall,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            1,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Err(
                    Derived(
                        UnresolvedLocalTerm,
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        BinaryOpnFirstArgumentTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Resolved(
                        ReducedTerm(
                            Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 22,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Resolved(
                            ReducedTerm(
                                Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 22,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 1,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 6,
                                variant: ImplicitType,
                            },
                        ),
                        unresolved_term_pattern: ImplicitSymbol,
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: FullyResolved(
                            ReducedTerm(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: Application {
                            function: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 21,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            argument: Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                        },
                        unresolved_term_pattern: Injection {
                            function: ModuleItem(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 21,
                                        },
                                    ),
                                ),
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ],
                        },
                        resolve_progress: Err(
                            Original(
                                UnresolvedTerm,
                            ),
                        ),
                    },
                ],
                first_unresolved_term: 0,
            },
            expectation_rules: LocalTermExpectationRules {
                arena: Arena {
                    data: [
                        LocalTermExpectationRule {
                            src_expr_idx: 0,
                            expectee: Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Resolved(
                                        ReducedTerm(
                                            Application(
                                                TermApplication(
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
                        LocalTermExpectationRule {
                            src_expr_idx: 1,
                            expectee: Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Resolved(
                                        ReducedTerm(
                                            Application(
                                                TermApplication(
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
                        LocalTermExpectationRule {
                            src_expr_idx: 2,
                            expectee: Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Resolved(
                                        ReducedTerm(
                                            Application(
                                                TermApplication(
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
                        LocalTermExpectationRule {
                            src_expr_idx: 3,
                            expectee: Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Resolved(
                                        ReducedTerm(
                                            Application(
                                                TermApplication(
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
                        LocalTermExpectationRule {
                            src_expr_idx: 4,
                            expectee: Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Resolved(
                                        ReducedTerm(
                                            Application(
                                                TermApplication(
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
                        LocalTermExpectationRule {
                            src_expr_idx: 5,
                            expectee: Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Resolved(
                                        ReducedTerm(
                                            Application(
                                                TermApplication(
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
    },
]