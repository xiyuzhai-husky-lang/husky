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
                        ExprError,
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
                        AsOperationRightOperandTermNotInferred,
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
        expr_terms: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(
                    Err(
                        Derived(
                            ExprError,
                        ),
                    ),
                ),
                None,
                None,
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [
            Term(`Type`),
            Term(`Slice $0`),
        ],
        current_symbol_tys: [],
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
                        BinaryOperationLeftOperandTypeNotInferred,
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
                        BinaryOperationLeftOperandTypeNotInferred,
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
        expr_terms: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [
            Term(`Type`),
            Term(`Slice $0`),
        ],
        current_symbol_tys: [],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 2,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 10,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Err(
                            Original(
                                UnresolvedTerm,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 17,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
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
                            src_expr_idx: 2,
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
                FormPath(`quick_sort::partition`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ExprError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        AsOperationRightOperandTermNotInferred,
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
                        BinaryOperationLeftOperandTypeNotInferred,
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
                        BinaryOperationLeftOperandTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ExprError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        AsOperationRightOperandTermNotInferred,
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
                    Unresolved(
                        UnresolvedTermIdx(
                            3,
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
                            5,
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
                    1,
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
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            6,
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
                        BinaryOperationLeftOperandTypeNotInferred,
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
                    Unresolved(
                        UnresolvedTermIdx(
                            7,
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
                        BinaryOperationLeftOperandTypeNotInferred,
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
                    Unresolved(
                        UnresolvedTermIdx(
                            8,
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
                        ExprError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        AsOperationRightOperandTermNotInferred,
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
                    Unresolved(
                        UnresolvedTermIdx(
                            10,
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
                            12,
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
                    2,
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
                    3,
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
                    4,
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
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            13,
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
                        BinaryOperationLeftOperandTypeNotInferred,
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
                    5,
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
        expr_terms: ArenaMap {
            data: [
                None,
                Some(
                    Err(
                        Derived(
                            ExprError,
                        ),
                    ),
                ),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(
                    Err(
                        Derived(
                            ExprError,
                        ),
                    ),
                ),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(
                    Err(
                        Derived(
                            ExprError,
                        ),
                    ),
                ),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [
            Term(`Type`),
            Term(`Slice $0`),
        ],
        current_symbol_tys: [],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 10,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 4,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Err(
                            Original(
                                UnresolvedTerm,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 9,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 17,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 20,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        2,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    2,
                                ),
                            ],
                        },
                        resolve_progress: Err(
                            Original(
                                UnresolvedTerm,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 18,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 20,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        4,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    4,
                                ),
                            ],
                        },
                        resolve_progress: Err(
                            Original(
                                UnresolvedTerm,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
                                ),
                                src_expr_idx: 21,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    5,
                                ),
                                src_expr_idx: 24,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    6,
                                ),
                                src_expr_idx: 27,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Err(
                            Original(
                                UnresolvedTerm,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    7,
                                ),
                                src_expr_idx: 34,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 20,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        9,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    9,
                                ),
                            ],
                        },
                        resolve_progress: Err(
                            Original(
                                UnresolvedTerm,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    8,
                                ),
                                src_expr_idx: 35,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 20,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        11,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    11,
                                ),
                            ],
                        },
                        resolve_progress: Err(
                            Original(
                                UnresolvedTerm,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    9,
                                ),
                                src_expr_idx: 40,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
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
                        LocalTermExpectationRule {
                            src_expr_idx: 19,
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
                        LocalTermExpectationRule {
                            src_expr_idx: 36,
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
                            variant: ImplicitlyConvertibleTo {
                                dst: Resolved(
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
                        LocalTermExpectationRule {
                            src_expr_idx: 37,
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
                            variant: ImplicitlyConvertibleTo {
                                dst: Resolved(
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
                        LocalTermExpectationRule {
                            src_expr_idx: 38,
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
                        LocalTermExpectationRule {
                            src_expr_idx: 44,
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
                            13,
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
                            13,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    10,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            14,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    11,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            15,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    12,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            16,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    13,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            17,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    14,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            18,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    15,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            19,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    16,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            20,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    17,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            21,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    18,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            22,
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    19,
                ),
                resolve_progress: Resolved(
                    LocalTermExpectationResolved {
                        implicit_conversion: None,
                        local_term: Unresolved(
                            UnresolvedTermIdx(
                                12,
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
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            23,
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
                    20,
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
                ty_result: Ok(
                    Resolved(
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
                                                    value: 21,
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
        expr_terms: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
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
            LocalTerm::Unresolved(
                UnresolvedTermIdx(
                    11,
                ),
            ),
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 22,
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
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 20,
                                },
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
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    11,
                                ),
                                src_expr_idx: 27,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    12,
                                ),
                                src_expr_idx: 15,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    13,
                                ),
                                src_expr_idx: 17,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    14,
                                ),
                                src_expr_idx: 18,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    15,
                                ),
                                src_expr_idx: 19,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    16,
                                ),
                                src_expr_idx: 20,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    17,
                                ),
                                src_expr_idx: 21,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    18,
                                ),
                                src_expr_idx: 22,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    19,
                                ),
                                src_expr_idx: 23,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    20,
                                ),
                                src_expr_idx: 24,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    21,
                                ),
                                src_expr_idx: 25,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                12,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 20,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        12,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    12,
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
                        LocalTermExpectationRule {
                            src_expr_idx: 16,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    13,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        12,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 17,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    14,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        12,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 18,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    15,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        12,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 19,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    16,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        12,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 20,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    17,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        12,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 21,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    18,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        12,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 22,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    19,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        12,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 23,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    20,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        12,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 24,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    21,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        12,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 25,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    22,
                                ),
                            ),
                            variant: ImplicitlyConvertibleTo {
                                dst: Unresolved(
                                    UnresolvedTermIdx(
                                        12,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                LocalTermExpectationResolved {
                                    implicit_conversion: None,
                                    local_term: Unresolved(
                                        UnresolvedTermIdx(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 28,
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
                    6,
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
                    7,
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
                    8,
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
                    9,
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
                    10,
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
                    11,
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
                ty_result: Ok(
                    Unresolved(
                        UnresolvedTermIdx(
                            3,
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
                    12,
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
                ty_result: Ok(
                    Resolved(
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
                                                    value: 21,
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
        expr_terms: ArenaMap {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
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
            LocalTerm::Unresolved(
                UnresolvedTermIdx(
                    1,
                ),
            ),
        ],
        unresolved_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 2,
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
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 20,
                                },
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
                    UnresolvedTermEntry {
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 17,
                                variant: ImplicitType,
                            },
                        ),
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
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 20,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        2,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    2,
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
                        LocalTermExpectationRule {
                            src_expr_idx: 10,
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
                                        2,
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
                            src_expr_idx: 11,
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
                                        2,
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
                            src_expr_idx: 12,
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
                                        2,
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
                            src_expr_idx: 13,
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
                                        2,
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
                            src_expr_idx: 14,
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
                                        2,
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
                            src_expr_idx: 15,
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
                                        2,
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
                            src_expr_idx: 18,
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
]