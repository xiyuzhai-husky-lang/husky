[
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 4,
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
                                    EqsRitchieCallType(
                                        ExpectEqsRitchieCallTypeResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 4,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            parameter_liasoned_tys: (),
                                            return_ty: (),
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 5,
                                            },
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
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
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
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 5,
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
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
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
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            4,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
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
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 13,
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
                                Err(
                                    Derived(
                                        Duplication(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            5,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            5,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            6,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 47,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
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
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            7,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 47,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
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
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 1,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 5,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 5,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: FullyResolved(
                            ReducedTerm(
                                Ritchie(
                                    TermRitchie(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 5,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
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
                        resolve_progress: Unresolved,
                    },
                ],
                first_unresolved_term: 0,
            },
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 0,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 4,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: EqsRitchieCallTy,
                            resolve_progress: Resolved(
                                Ok(
                                    EqsRitchieCallType(
                                        ExpectEqsRitchieCallTypeResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 4,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            parameter_liasoned_tys: (),
                                            return_ty: (),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 4,
                            expectee: Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 13,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
                                    ReducedTerm(
                                        Application(
                                            TermApplication(
                                                Id {
                                                    value: 14,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Original(
                                        Todo,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 1,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: ImplicitlyConversion {
                                destination: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 2,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: ImplicitlyConversion {
                                destination: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 3,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: ImplicitlyConversion {
                                destination: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 5,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
                                    ReducedTerm(
                                        Application(
                                            TermApplication(
                                                Id {
                                                    value: 16,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Original(
                                        Todo,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 6,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
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
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 47,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 7,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
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
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 47,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
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
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
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
                None,
                None,
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                TodoSuffix,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 4,
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
                                    EqsRitchieCallType(
                                        ExpectEqsRitchieCallTypeResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 4,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            parameter_liasoned_tys: (),
                                            return_ty: (),
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 13,
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
                                Err(
                                    Derived(
                                        Duplication(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            2,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            2,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 47,
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
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 47,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
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
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 47,
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
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 47,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
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
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                FieldTypeError(
                                    Todo,
                                ),
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    2,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            5,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                            6,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                TodoSuffix,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            7,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            7,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    3,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            8,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                            9,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            10,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            10,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    4,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            11,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                None,
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                TodoScopeResolution,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            13,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            13,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    5,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            14,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                            15,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 17,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            16,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 17,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
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
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 17,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            17,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 17,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 17,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
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
                Some(
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
                            18,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                None,
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    7,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            19,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Unresolved(
                                                UnresolvedTermIdx(
                                                    6,
                                                ),
                                            ),
                                            destination: Unresolved(
                                                UnresolvedTermIdx(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    8,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            20,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                None,
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    10,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            21,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Unresolved(
                                                UnresolvedTermIdx(
                                                    9,
                                                ),
                                            ),
                                            destination: Unresolved(
                                                UnresolvedTermIdx(
                                                    9,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    11,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            22,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                None,
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    13,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            23,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Unresolved(
                                                UnresolvedTermIdx(
                                                    12,
                                                ),
                                            ),
                                            destination: Unresolved(
                                                UnresolvedTermIdx(
                                                    12,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    14,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            24,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
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
                            25,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            26,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 47,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
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
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                FieldTypeError(
                                    Todo,
                                ),
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    15,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            27,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                            28,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
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
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                TodoSuffix,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            30,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            30,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    16,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            29,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            31,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            31,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    17,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            32,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                            33,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                None,
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                TodoScopeResolution,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                EntityTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    18,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            34,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                            35,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
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
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                TodoSuffix,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
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
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                TodoSuffix,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
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
                            36,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    19,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            37,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                            38,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                None,
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    21,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            39,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    22,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            40,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                MethodOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    23,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            41,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                            42,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
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
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                TodoSuffix,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
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
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                TodoSuffix,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
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
                            43,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
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
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                TodoSuffix,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    24,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            44,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                            45,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                None,
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    26,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            46,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                MethodOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                None,
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    28,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            47,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    29,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            48,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                MethodOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
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
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                TodoSuffix,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    30,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            49,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                            50,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            51,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            51,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            52,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            52,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    31,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            53,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                PrefixOperandTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
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
                            54,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    32,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            55,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                            56,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                None,
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Original(
                                TodoScopeResolution,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BlockTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
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
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`FermiMatchResult`),
            ),
        ],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 26,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 12,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 12,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 12,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
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
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 16,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 16,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 29,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 29,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 33,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 33,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 40,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
                                ),
                                src_expr_idx: 40,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 48,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    5,
                                ),
                                src_expr_idx: 48,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 47,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    6,
                                ),
                                src_expr_idx: 47,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    6,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                6,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 48,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        6,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    6,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 52,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    7,
                                ),
                                src_expr_idx: 52,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 51,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    8,
                                ),
                                src_expr_idx: 51,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    9,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                9,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 52,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
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
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 56,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    9,
                                ),
                                src_expr_idx: 56,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 55,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    10,
                                ),
                                src_expr_idx: 55,
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
                        src_expr_idx: 56,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
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
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 61,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    11,
                                ),
                                src_expr_idx: 61,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 74,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    12,
                                ),
                                src_expr_idx: 74,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 80,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    13,
                                ),
                                src_expr_idx: 80,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 94,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    14,
                                ),
                                src_expr_idx: 94,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 139,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    15,
                                ),
                                src_expr_idx: 139,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 148,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    16,
                                ),
                                src_expr_idx: 148,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 146,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    17,
                                ),
                                src_expr_idx: 146,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 148,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        20,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    20,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 153,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    18,
                                ),
                                src_expr_idx: 153,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 209,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    19,
                                ),
                                src_expr_idx: 209,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 216,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    20,
                                ),
                                src_expr_idx: 216,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 216,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        25,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    25,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 226,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    21,
                                ),
                                src_expr_idx: 226,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 224,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    22,
                                ),
                                src_expr_idx: 224,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 226,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        27,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    27,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 241,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    23,
                                ),
                                src_expr_idx: 241,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 253,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    24,
                                ),
                                src_expr_idx: 253,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 258,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    25,
                                ),
                                src_expr_idx: 258,
                                variant: UnspecifiedFloatType,
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
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 10,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 4,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: EqsRitchieCallTy,
                            resolve_progress: Resolved(
                                Ok(
                                    EqsRitchieCallType(
                                        ExpectEqsRitchieCallTypeResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 4,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            parameter_liasoned_tys: (),
                                            return_ty: (),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 11,
                            expectee: Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 13,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
                                    ReducedTerm(
                                        Application(
                                            TermApplication(
                                                Id {
                                                    value: 14,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Original(
                                        Todo,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 12,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
                                    ReducedTerm(
                                        Application(
                                            TermApplication(
                                                Id {
                                                    value: 16,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Original(
                                        Todo,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 13,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
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
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 47,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 14,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
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
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 47,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 16,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    2,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 17,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 27,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Derived(
                                        TermTypeError {
                                            term: Entity(
                                                ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 32,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            error: Derived(
                                                SignatureError,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 29,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    3,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 30,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 31,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Derived(
                                        TermTypeError {
                                            term: Entity(
                                                ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 32,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            error: Derived(
                                                SignatureError,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 33,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    4,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 34,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 38,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Derived(
                                        TermTypeError {
                                            term: Entity(
                                                ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 32,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            error: Derived(
                                                SignatureError,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 40,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    5,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 41,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 42,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 17,
                                                    },
                                                ),
                                            ),
                                        ),
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
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 17,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 43,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 17,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
                                    ReducedTerm(
                                        Entity(
                                            ModuleItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 17,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 17,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 17,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 47,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    7,
                                ),
                            ),
                            expectation: ImplicitlyConversion {
                                destination: Unresolved(
                                    UnresolvedTermIdx(
                                        6,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Unresolved(
                                                UnresolvedTermIdx(
                                                    6,
                                                ),
                                            ),
                                            destination: Unresolved(
                                                UnresolvedTermIdx(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 48,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    8,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 51,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    10,
                                ),
                            ),
                            expectation: ImplicitlyConversion {
                                destination: Unresolved(
                                    UnresolvedTermIdx(
                                        9,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Unresolved(
                                                UnresolvedTermIdx(
                                                    9,
                                                ),
                                            ),
                                            destination: Unresolved(
                                                UnresolvedTermIdx(
                                                    9,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 52,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    11,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 55,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    13,
                                ),
                            ),
                            expectation: ImplicitlyConversion {
                                destination: Unresolved(
                                    UnresolvedTermIdx(
                                        12,
                                    ),
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Unresolved(
                                                UnresolvedTermIdx(
                                                    12,
                                                ),
                                            ),
                                            destination: Unresolved(
                                                UnresolvedTermIdx(
                                                    12,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 56,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    14,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 58,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 59,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
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
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 47,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 61,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    15,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 62,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 74,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    16,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 73,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Derived(
                                        TermTypeError {
                                            term: Entity(
                                                ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 32,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            error: Derived(
                                                SignatureError,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 76,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Derived(
                                        TermTypeError {
                                            term: Entity(
                                                ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 32,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            error: Derived(
                                                SignatureError,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 80,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    17,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 81,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 94,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    18,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 95,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 131,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 139,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    19,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 140,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 146,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    21,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 148,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    22,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 153,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    23,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 154,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 184,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 209,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    24,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 210,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 216,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    26,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 224,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    28,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 226,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    29,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 241,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    30,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 242,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 243,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Derived(
                                        TermTypeError {
                                            term: Entity(
                                                ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 32,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            error: Derived(
                                                SignatureError,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 245,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Derived(
                                        TermTypeError {
                                            term: Entity(
                                                ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 32,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            error: Derived(
                                                SignatureError,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 253,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    31,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 256,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 258,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    32,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 259,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 15,
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
                                Err(
                                    Derived(
                                        Duplication(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                MethodOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            1,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BlockTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
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
                None,
                None,
            ],
        },
        inherited_symbol_tys: [
            Term(`Ref TermLiteral::EvalLifetime ConcaveComponent`),
        ],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 1,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 4,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 4,
                                variant: UnspecifiedFloatType,
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
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 0,
                            expectee: Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Original(
                                        TermTypeError {
                                            term: Application(
                                                TermApplication(
                                                    Id {
                                                        value: 15,
                                                    },
                                                ),
                                            ),
                                            error: Original(
                                                Todo,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 4,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 5,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            Term(`Option f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 15,
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
                                Err(
                                    Derived(
                                        Duplication(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                MethodOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            1,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 15,
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
                                Err(
                                    Derived(
                                        Duplication(
                                            3,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                MethodOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                PrefixOperandTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BlockTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
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
                None,
                None,
                None,
                None,
            ],
        },
        inherited_symbol_tys: [
            Term(`Ref TermLiteral::EvalLifetime ConcaveComponent`),
        ],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 1,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 4,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 4,
                                variant: UnspecifiedFloatType,
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
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 0,
                            expectee: Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Original(
                                        TermTypeError {
                                            term: Application(
                                                TermApplication(
                                                    Id {
                                                        value: 15,
                                                    },
                                                ),
                                            ),
                                            error: Original(
                                                Todo,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 4,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 5,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 6,
                            expectee: Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Original(
                                        TermTypeError {
                                            term: Application(
                                                TermApplication(
                                                    Id {
                                                        value: 15,
                                                    },
                                                ),
                                            ),
                                            error: Original(
                                                Todo,
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
        return_ty: Some(
            Term(`Option f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`mnist_classifier::digits::one::hat`, `Function`),
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 15,
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
                                Err(
                                    Derived(
                                        Duplication(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                MethodOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            1,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            3,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                Some(
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
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                PrefixOperandTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BlockTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
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
            Term(`Ref TermLiteral::EvalLifetime ConcaveComponent`),
        ],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 2,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 4,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 4,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 8,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 8,
                                variant: UnspecifiedFloatType,
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
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 0,
                            expectee: Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Err(
                                    Original(
                                        TermTypeError {
                                            term: Application(
                                                TermApplication(
                                                    Id {
                                                        value: 15,
                                                    },
                                                ),
                                            ),
                                            error: Original(
                                                Todo,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 4,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 5,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 8,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 9,
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
                            expectation: ImplicitlyConversion {
                                destination: Resolved(
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
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
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
                                            destination: Resolved(
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
                                ),
                            ),
                        },
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            Term(`Option f32`),
        ),
        self_ty: None,
    },
]