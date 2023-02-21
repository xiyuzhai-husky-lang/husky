[
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
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
                                                value: 5,
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
            ],
        },
        inherited_symbol_tys: [
            Term(`Ref TermLiteral::EvalLifetime RawContour`),
        ],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 2,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 3,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 3,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 6,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 6,
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
                                                value: 5,
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
                                                        value: 5,
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
                            src_expr_idx: 3,
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
                            src_expr_idx: 4,
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
                FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
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
                                                        value: 19,
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
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 19,
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
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
                        ty_result: Err(
                            Derived(
                                BinaryOperationRightOperandTypeNotInferred,
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
                                                        value: 19,
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
                                                                        value: 19,
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
                                BracketedItemTypeError,
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
                                                        value: 19,
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
                                                                        value: 19,
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
                            Unresolved,
                        ),
                    },
                ),
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                BinaryOperationRightOperandTypeNotInferred,
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
                                BinaryOperationRightOperandTypeNotInferred,
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
                                BracketedItemTypeError,
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
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 19,
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
                                                                        value: 19,
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
                                BracketedItemTypeError,
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
                                BinaryOperationRightOperandTypeNotInferred,
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
                                    2,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            7,
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
                                BracketedItemTypeError,
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
                                BracketedItemTypeError,
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
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 19,
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
                                                                        value: 19,
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
                                BracketedItemTypeError,
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
                                BinaryOperationRightOperandTypeNotInferred,
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                                    6,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            12,
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
                                BracketedItemTypeError,
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
                                    7,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            13,
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
                                BracketedItemTypeError,
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
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 19,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            11,
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
                                                                        value: 19,
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
                                BracketedItemTypeError,
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
                                BinaryOperationRightOperandTypeNotInferred,
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
                                                        value: 3,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            14,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 2,
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
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
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
            Term(`r32`),
            Term(`r32`),
        ],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 8,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 1,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 1,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 6,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 6,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 15,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 15,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 20,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 20,
                                variant: UnspecifiedIntegerType,
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
                                    4,
                                ),
                                src_expr_idx: 33,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 49,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    5,
                                ),
                                src_expr_idx: 49,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 35,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    6,
                                ),
                                src_expr_idx: 35,
                                variant: UnspecifiedIntegerType,
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
                                    7,
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
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 11,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 19,
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
                                                                        value: 19,
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
                            src_expr_idx: 3,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 19,
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
                                                                        value: 19,
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
                            src_expr_idx: 0,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 19,
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
                                                                        value: 19,
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
                            src_expr_idx: 1,
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
                                                        value: 19,
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
                                                                        value: 19,
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
                            src_expr_idx: 25,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 19,
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
                                                                        value: 19,
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
                            src_expr_idx: 15,
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
                            src_expr_idx: 20,
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
                            src_expr_idx: 33,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            src_expr_idx: 45,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 19,
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
                                                                        value: 19,
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
                            src_expr_idx: 35,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    6,
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
                            src_expr_idx: 40,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    7,
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
                            src_expr_idx: 49,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            src_expr_idx: 51,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
            Term(`r32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
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
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                CallableTypeError,
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
                                ApplicationArgumentTypeNotInferred,
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
                                InheritedSymbolTypeError,
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
                            0,
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
                                    2,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    4,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            2,
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
                                ExprError,
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
                                CallableTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 1,
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
                                    EqsRitchieCallType(
                                        ExpectEqsRitchieCallTypeResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 1,
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
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 19,
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
                                                                        value: 19,
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
                                BinaryOperationRightOperandTypeNotInferred,
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
                                    7,
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 19,
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
                                                                        value: 19,
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
                                                        value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            8,
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
                            10,
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
                            11,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            11,
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
                                    InsSort(
                                        ExpectInsSortResolvedOk {
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
                                                        value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    10,
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
                                    11,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            15,
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
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 1,
                                            },
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
                                    EqsRitchieCallType(
                                        ExpectEqsRitchieCallTypeResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 1,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    13,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            17,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    14,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            18,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            18,
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
                            19,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            19,
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
                                                        value: 19,
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
                                                                        value: 19,
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
                            21,
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
                            22,
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
                            23,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            23,
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
                            24,
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
                                                        value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                                    20,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            26,
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
                                    21,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            27,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            27,
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
                                                        value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                                    23,
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 1,
                                            },
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
                                Ok(
                                    EqsRitchieCallType(
                                        ExpectEqsRitchieCallTypeResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 1,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    27,
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
                                    25,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    28,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            33,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            33,
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
                                                        value: 19,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            34,
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
                                                                        value: 19,
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
                                Err(
                                    Derived(
                                        Duplication(
                                            36,
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
                            37,
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
                                                        value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            39,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            39,
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
                                                        value: 3,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            40,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                None,
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    33,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    38,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            43,
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    36,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            42,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            42,
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
                                BracketedItemTypeError,
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
                                                        value: 3,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            44,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                                FunctionTypeNotInferredInApplicationOrFunctionCall,
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 2,
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
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
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
                Term(`bool`),
            ),
        ],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 26,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 7,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 7,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 11,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
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
                        src_expr_idx: 11,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        1,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 14,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 14,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 14,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        3,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    3,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 28,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 28,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 26,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
                                ),
                                src_expr_idx: 26,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 26,
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
                        src_expr_idx: 34,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    5,
                                ),
                                src_expr_idx: 34,
                                variant: ExprEvalLifetime,
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
                                    6,
                                ),
                                src_expr_idx: 40,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 38,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    7,
                                ),
                                src_expr_idx: 38,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 40,
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
                        src_expr_idx: 48,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    8,
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
                        src_expr_idx: 44,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    9,
                                ),
                                src_expr_idx: 44,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
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
                        src_expr_idx: 49,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    10,
                                ),
                                src_expr_idx: 49,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 49,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        15,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    15,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 60,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    11,
                                ),
                                src_expr_idx: 60,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 67,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    12,
                                ),
                                src_expr_idx: 67,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 65,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    13,
                                ),
                                src_expr_idx: 65,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 63,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    14,
                                ),
                                src_expr_idx: 63,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 65,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        19,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    19,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 70,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    15,
                                ),
                                src_expr_idx: 70,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 70,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        22,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    22,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 78,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    16,
                                ),
                                src_expr_idx: 78,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 78,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        24,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    24,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 79,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    17,
                                ),
                                src_expr_idx: 79,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 76,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    18,
                                ),
                                src_expr_idx: 76,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 79,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        26,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    26,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 88,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    19,
                                ),
                                src_expr_idx: 88,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 93,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    20,
                                ),
                                src_expr_idx: 93,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 91,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    21,
                                ),
                                src_expr_idx: 91,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 91,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        31,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    31,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 97,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    22,
                                ),
                                src_expr_idx: 97,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 107,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    23,
                                ),
                                src_expr_idx: 107,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 105,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    24,
                                ),
                                src_expr_idx: 105,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 105,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        35,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    35,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 103,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    25,
                                ),
                                src_expr_idx: 103,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 103,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        37,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    37,
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
                            src_expr_idx: 7,
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
                            src_expr_idx: 11,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    2,
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
                                Err(
                                    Original(
                                        Todo,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 14,
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
                            src_expr_idx: 26,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    7,
                                ),
                            ),
                            expectation: EqsRefMutApplication {
                                lifetime: Unresolved(
                                    UnresolvedTermIdx(
                                        5,
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
                            src_expr_idx: 21,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 1,
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
                                                                value: 1,
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
                            src_expr_idx: 22,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 19,
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
                                                                        value: 19,
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
                                                        value: 19,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    1,
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
                                                                        value: 19,
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
                            src_expr_idx: 28,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            src_expr_idx: 29,
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
                            src_expr_idx: 32,
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
                            expectation: EqsRefMutApplication {
                                lifetime: Unresolved(
                                    UnresolvedTermIdx(
                                        8,
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
                            src_expr_idx: 33,
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
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    1,
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
                            src_expr_idx: 34,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    10,
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
                            src_expr_idx: 40,
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
                            src_expr_idx: 41,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 1,
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
                                                                value: 1,
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
                            src_expr_idx: 44,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    13,
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
                            src_expr_idx: 48,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    14,
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
                                                            value: 19,
                                                        },
                                                    ),
                                                ),
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
                            src_expr_idx: 49,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    16,
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
                                                            value: 19,
                                                        },
                                                    ),
                                                ),
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
                            src_expr_idx: 51,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 19,
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
                                                                        value: 19,
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
                            src_expr_idx: 54,
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
                            src_expr_idx: 57,
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
                            expectation: EqsRefMutApplication {
                                lifetime: Unresolved(
                                    UnresolvedTermIdx(
                                        17,
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
                            src_expr_idx: 59,
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
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    1,
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
                            src_expr_idx: 60,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            src_expr_idx: 63,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    20,
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
                            src_expr_idx: 65,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    21,
                                ),
                            ),
                            expectation: EqsRefMutApplication {
                                lifetime: Unresolved(
                                    UnresolvedTermIdx(
                                        18,
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
                            src_expr_idx: 67,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            src_expr_idx: 70,
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
                            src_expr_idx: 71,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 1,
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
                                                                value: 1,
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
                            src_expr_idx: 78,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    25,
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
                                                            value: 19,
                                                        },
                                                    ),
                                                ),
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
                            src_expr_idx: 76,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    27,
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
                            src_expr_idx: 79,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    28,
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
                                                            value: 19,
                                                        },
                                                    ),
                                                ),
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
                            src_expr_idx: 81,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 19,
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
                                                                        value: 19,
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
                            src_expr_idx: 85,
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
                            src_expr_idx: 86,
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
                            expectation: EqsRefMutApplication {
                                lifetime: Unresolved(
                                    UnresolvedTermIdx(
                                        29,
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
                            src_expr_idx: 87,
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
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    1,
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
                            src_expr_idx: 88,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            src_expr_idx: 91,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    32,
                                ),
                            ),
                            expectation: EqsRefMutApplication {
                                lifetime: Unresolved(
                                    UnresolvedTermIdx(
                                        30,
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
                            src_expr_idx: 93,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            src_expr_idx: 97,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    33,
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
                            src_expr_idx: 105,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    36,
                                ),
                            ),
                            expectation: EqsRefMutApplication {
                                lifetime: Unresolved(
                                    UnresolvedTermIdx(
                                        34,
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
                            src_expr_idx: 103,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    38,
                                ),
                            ),
                            expectation: InsSort {
                                smallest_universe: TermUniverse(
                                    1,
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 107,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            src_expr_idx: 114,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
            Term(`List ConnectedComponent`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::connected_component`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                    },
                    ident: `raw_contours`,
                },
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
                                                value: 2,
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
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 2,
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
                        ty_result: Err(
                            Derived(
                                SelfTypeNotInferredForSelfValue,
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
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 8,
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
            ],
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
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 0,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 2,
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
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 2,
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
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 8,
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
                                                        value: 8,
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
            Term(`List RawContour`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::connected_component`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                    },
                    ident: `eff_holes`,
                },
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                SelfTypeNotInferredForSelfValue,
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
                                MethodOwnerTypeNotInferred,
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
                                CallableTypeError,
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
                None,
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
                                ApplicationArgumentTypeNotInferred,
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
                                    Ritchie(
                                        TermRitchie(
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
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 3,
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
                                    Ritchie(
                                        TermRitchie(
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
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 3,
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
                                    Ritchie(
                                        TermRitchie(
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
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 3,
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
                                MethodOwnerTypeNotInferred,
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
                            4,
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
                            Original(
                                TypeCallTypeError(
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
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
                None,
                None,
                None,
                None,
                None,
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
                    Ok(
                        Resolved(
                            ReducedTerm(
                                Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 31,
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
                            expectation: EqsRitchieCallTy,
                            resolve_progress: Resolved(
                                Err(
                                    Original(
                                        Todo,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 10,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 3,
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
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 3,
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
                            src_expr_idx: 14,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 3,
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
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 3,
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
                            src_expr_idx: 19,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 3,
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
                                Ok(
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 3,
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
                            src_expr_idx: 22,
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
                        LocalTermExpectationEntry {
                            src_expr_idx: 25,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
            Term(`EffHoles`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::connected_component`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                    },
                    ident: `max_hole_ilen`,
                },
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
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
                            0,
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
                                SelfTypeNotInferredForSelfValue,
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
                            1,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
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
                None,
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
                                                        value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            5,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
                None,
                None,
                None,
                None,
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
                    Ok(
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
                ),
                None,
                None,
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 4,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 0,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 0,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 3,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 3,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 11,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
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
                        src_expr_idx: 11,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
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
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 19,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 19,
                                variant: ExprEvalLifetime,
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
                            src_expr_idx: 3,
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
                            src_expr_idx: 11,
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
                            src_expr_idx: 16,
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
                            src_expr_idx: 19,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            src_expr_idx: 21,
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
                            expectation: EqsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
                        LocalTermExpectationEntry {
                            src_expr_idx: 23,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
            Term(`f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::connected_component`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                    },
                    ident: `max_row_span`,
                },
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
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
                            0,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
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
                            1,
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
                                    2,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            2,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    5,
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
                                MethodOwnerTypeNotInferred,
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
                                                        value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            5,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
                None,
                None,
                None,
                None,
                None,
                None,
                None,
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
                                                    value: 17,
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
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 5,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 0,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 0,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 1,
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
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 4,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 4,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 14,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 14,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 10,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
                                ),
                                src_expr_idx: 10,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 10,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
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
                            src_expr_idx: 1,
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
                            src_expr_idx: 4,
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
                            src_expr_idx: 10,
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
                            src_expr_idx: 14,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            expectation: EqsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
                        LocalTermExpectationEntry {
                            src_expr_idx: 18,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
            Term(`f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::connected_component`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                    },
                    ident: `row_span_sum`,
                },
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
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
                            0,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
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
                            1,
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
                                    2,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            2,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                None,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    5,
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            5,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
                None,
                None,
                None,
                None,
                None,
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
                                                    value: 17,
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
            ],
        },
        inherited_symbol_tys: [],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 5,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 0,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 0,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 1,
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
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 4,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 4,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 12,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 12,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 9,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
                                ),
                                src_expr_idx: 9,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 9,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
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
                            src_expr_idx: 1,
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
                            src_expr_idx: 4,
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
                            src_expr_idx: 9,
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
                            src_expr_idx: 12,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                            expectation: EqsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
                        LocalTermExpectationEntry {
                            src_expr_idx: 16,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
            Term(`f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::connected_component`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                    },
                    ident: `distribution`,
                },
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
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
                            0,
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
                                    2,
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
                                    3,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            2,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    5,
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
                                    7,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            6,
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
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                None,
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
                None,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    10,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            7,
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            8,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    11,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            9,
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
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
                None,
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
                None,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    14,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            10,
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            11,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            12,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            12,
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
                                CallableTypeError,
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
                                                        value: 2,
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
                                                                        value: 2,
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
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
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
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 11,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 0,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 0,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 4,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 4,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 4,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        1,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 6,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 6,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 11,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
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
                        src_expr_idx: 11,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
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
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 17,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
                                ),
                                src_expr_idx: 17,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 19,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    5,
                                ),
                                src_expr_idx: 19,
                                variant: UnspecifiedIntegerType,
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
                                    6,
                                ),
                                src_expr_idx: 33,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 30,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    7,
                                ),
                                src_expr_idx: 30,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 30,
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
                        src_expr_idx: 34,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    8,
                                ),
                                src_expr_idx: 34,
                                variant: UnspecifiedIntegerType,
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
                                    9,
                                ),
                                src_expr_idx: 48,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 45,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    10,
                                ),
                                src_expr_idx: 45,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 45,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        13,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    13,
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
                            src_expr_idx: 4,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    2,
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
                                Err(
                                    Original(
                                        Todo,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 6,
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
                            src_expr_idx: 11,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    5,
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
                                Err(
                                    Original(
                                        Todo,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 12,
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
                            src_expr_idx: 17,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    6,
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
                            src_expr_idx: 19,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    7,
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
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    10,
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
                            src_expr_idx: 33,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            src_expr_idx: 34,
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
                            src_expr_idx: 45,
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
                            src_expr_idx: 48,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            src_expr_idx: 49,
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
                            expectation: EqsRitchieCallTy,
                            resolve_progress: Resolved(
                                Err(
                                    Original(
                                        Todo,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 55,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
            Term(`ConnectedComponentDistribution`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::connected_component`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                    },
                    ident: `upper_mass`,
                },
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                SelfTypeNotInferredForSelfValue,
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
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
                Some(
                    Ok(
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
                ),
                None,
                None,
            ],
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
                    data: [
                        LocalTermExpectationEntry {
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
                            expectation: EqsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
        return_ty: Some(
            Term(`f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::connected_component`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                    },
                    ident: `lower_mass`,
                },
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                SelfTypeNotInferredForSelfValue,
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
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
                Some(
                    Ok(
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
                ),
                None,
                None,
            ],
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
                    data: [
                        LocalTermExpectationEntry {
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
                            expectation: EqsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
        return_ty: Some(
            Term(`f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::connected_component`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                    },
                    ident: `top_k_row_span_sum`,
                },
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
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
                            0,
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
                                                        value: 13,
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
                                                                        value: 13,
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
                                                        value: 13,
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
                                                                        value: 13,
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
                                                                        value: 13,
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            4,
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
                                    3,
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
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 13,
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
                                                                        value: 13,
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
                None,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    6,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            7,
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            8,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            9,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
                None,
                None,
                None,
                None,
                None,
                None,
                None,
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
                    Ok(
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
                ),
                None,
                None,
            ],
        },
        inherited_symbol_tys: [
            Term(`i32`),
        ],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 5,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 0,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 0,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 4,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 4,
                                variant: UnspecifiedIntegerType,
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
                                    2,
                                ),
                                src_expr_idx: 8,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 8,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
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
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 22,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 22,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 19,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
                                ),
                                src_expr_idx: 19,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 19,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        5,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    5,
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
                            src_expr_idx: 1,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 13,
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
                                                                        value: 13,
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
                            src_expr_idx: 2,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 13,
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
                                                            value: 13,
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
                                                                        value: 13,
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
                                                                        value: 13,
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
                            src_expr_idx: 3,
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
                            src_expr_idx: 4,
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
                            src_expr_idx: 8,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    3,
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
                                Err(
                                    Original(
                                        Todo,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 12,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 13,
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
                                                                        value: 13,
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
                            src_expr_idx: 19,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    6,
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
                            src_expr_idx: 22,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            src_expr_idx: 24,
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
                            expectation: EqsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
                        LocalTermExpectationEntry {
                            src_expr_idx: 26,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
            Term(`f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::connected_component`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                    },
                    ident: `top_k_row_right_mass_sum`,
                },
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
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
                            0,
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
                                                        value: 13,
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
                                                                        value: 13,
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
                                                        value: 13,
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
                                                                        value: 13,
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
                                                                        value: 13,
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            4,
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
                                    3,
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
                        ty_result: Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                        expectation_rule_idx: None,
                        resolve_progress: Unresolved,
                    },
                ),
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 13,
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
                                                                        value: 13,
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
                None,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    6,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            7,
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            8,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            9,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
                None,
                None,
                None,
                None,
                None,
                None,
                None,
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
                    Ok(
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
                ),
                None,
                None,
            ],
        },
        inherited_symbol_tys: [
            Term(`i32`),
        ],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 5,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 0,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 0,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 4,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 4,
                                variant: UnspecifiedIntegerType,
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
                                    2,
                                ),
                                src_expr_idx: 8,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 8,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
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
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 22,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 22,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 19,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
                                ),
                                src_expr_idx: 19,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 19,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        5,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    5,
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
                            src_expr_idx: 1,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 13,
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
                                                                        value: 13,
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
                            src_expr_idx: 2,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 13,
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
                                                            value: 13,
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
                                                                        value: 13,
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
                                                                        value: 13,
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
                            src_expr_idx: 3,
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
                            src_expr_idx: 4,
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
                            src_expr_idx: 8,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    3,
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
                                Err(
                                    Original(
                                        Todo,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 12,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 13,
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
                                                                        value: 13,
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
                            src_expr_idx: 19,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    6,
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
                            src_expr_idx: 22,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 3,
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
                                                            value: 3,
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
                                                                        value: 3,
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
                                                                        value: 3,
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
                            src_expr_idx: 24,
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
                            expectation: EqsSort {
                                smallest_universe: TermUniverse(
                                    0,
                                ),
                            },
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        ExpectEqsSortResolvedOk {
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
                        LocalTermExpectationEntry {
                            src_expr_idx: 26,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 2,
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
                                                                        value: 2,
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
            Term(`f32`),
        ),
        self_ty: None,
    },
]