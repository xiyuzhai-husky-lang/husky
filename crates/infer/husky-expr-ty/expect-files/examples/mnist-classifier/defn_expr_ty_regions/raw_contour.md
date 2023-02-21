[
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
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
            ],
        },
        inherited_symbol_tys: [
            Term(`r32`),
            Term(`i32`),
        ],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 1,
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
                ],
                first_unresolved_term: 0,
            },
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
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
                            src_expr_idx: 7,
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
                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
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
        inherited_symbol_tys: [
            Term(`r32`),
            Term(`i32`),
        ],
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
                            src_expr_idx: 4,
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
                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
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
            ],
        },
        inherited_symbol_tys: [
            Term(`r32`),
            Term(`i32`),
        ],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 1,
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
                ],
                first_unresolved_term: 0,
            },
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
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
                            src_expr_idx: 7,
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
                FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
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
                                                value: 12,
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
                                                                value: 12,
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
                            1,
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
                                                                        value: 19,
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
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 12,
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
                                                                value: 12,
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
                                                                        value: 19,
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
            Term(`r32`),
            Term(`r32`),
            Term(`i32`),
        ],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`r32`),
            ),
            LocalTerm::Resolved(
                Term(`r32`),
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
                            src_expr_idx: 0,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 12,
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
                                                                value: 12,
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
                            src_expr_idx: 1,
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
                                                                        value: 19,
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
                            src_expr_idx: 4,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 12,
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
                                                                value: 12,
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
                                                                        value: 19,
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
                            src_expr_idx: 7,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            Term(`Direction`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
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
                                                        value: 34,
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
                            Unresolved,
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
                            1,
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
                                                        value: 34,
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
                            Unresolved,
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
                            3,
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
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
                Some(
                    Ok(
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
                ),
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
                                                    value: 13,
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
                                                    value: 19,
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
                None,
                None,
            ],
        },
        inherited_symbol_tys: [
            Term(`Direction`),
            Term(`Direction`),
        ],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 1,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 13,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 13,
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
                            src_expr_idx: 10,
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
                            src_expr_idx: 1,
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
                            src_expr_idx: 0,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            expectation: ExplicitlyConvertible(
                                ExpectExplicitlyConvertible {
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
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 4,
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
                            src_expr_idx: 3,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            expectation: ExplicitlyConvertible(
                                ExpectExplicitlyConvertible {
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
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 13,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            Term(`i32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
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
                                                value: 12,
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
                                                                value: 12,
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
                            1,
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
                                                                        value: 19,
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
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 12,
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
                                                                value: 12,
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
                                                                        value: 19,
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
            Term(`r32`),
            Term(`r32`),
            Term(`i32`),
            Term(`Direction`),
        ],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`r32`),
            ),
            LocalTerm::Resolved(
                Term(`r32`),
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
                            src_expr_idx: 0,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 12,
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
                                                                value: 12,
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
                            src_expr_idx: 1,
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
                                                                        value: 19,
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
                            src_expr_idx: 4,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 12,
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
                                                                value: 12,
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
                                                                        value: 19,
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
                            src_expr_idx: 7,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            Term(`Direction`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
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
                                                value: 10,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    6,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    7,
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
            ],
        },
        inherited_symbol_tys: [
            Term(`List Point2d`),
        ],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 6,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 6,
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
                        src_expr_idx: 6,
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
                        src_expr_idx: 9,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
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
                        src_expr_idx: 19,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
                                ),
                                src_expr_idx: 19,
                                variant: UnspecifiedFloatType,
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
                                    5,
                                ),
                                src_expr_idx: 26,
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
                                                value: 10,
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
                                                        value: 10,
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
                            src_expr_idx: 6,
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
                            src_expr_idx: 11,
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
                            src_expr_idx: 26,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            Term(`Point2d`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::Entity(
                FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
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
                None,
                None,
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
                                    3,
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
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    4,
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
                                    8,
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    11,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            8,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            8,
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
                                    12,
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
                            11,
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
                                    15,
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
                                    17,
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
                                    19,
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
                                    20,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            16,
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
                                    22,
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 13,
                                            },
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
                                    EqsRitchieCallType(
                                        ExpectEqsRitchieCallTypeResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 13,
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            19,
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
                                                                        value: 34,
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
                                                        value: 34,
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
                                                                        value: 34,
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
                                    23,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            21,
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
                                    24,
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
                            23,
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
                                    26,
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
                                    27,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            25,
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
                                    28,
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
                            27,
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
                                                        value: 34,
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
                                                                        value: 34,
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
                                                        value: 34,
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
                                                                        value: 34,
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
                                                                        value: 34,
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
                            29,
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
                            32,
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
                            34,
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
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 14,
                                            },
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
                                    EqsRitchieCallType(
                                        ExpectEqsRitchieCallTypeResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Ritchie(
                                                        TermRitchie(
                                                            Id {
                                                                value: 14,
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
                        ty_result: Ok(
                            Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
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
                                                                        value: 34,
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
                                                                        value: 34,
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
                                                        value: 34,
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
                                    InsSort(
                                        ExpectInsSortResolvedOk {
                                            destination: Resolved(
                                                ReducedTerm(
                                                    Entity(
                                                        ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 34,
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
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            39,
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
                                                                value: 15,
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
                                                        value: 34,
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
                                                                        value: 34,
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
                                                                        value: 34,
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
                                                        value: 34,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            41,
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
                                                                        value: 34,
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
                                                                        value: 34,
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
                            42,
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
                None,
                None,
                None,
                None,
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
                            44,
                        ),
                        resolve_progress: Expected(
                            Unresolved,
                        ),
                    },
                ),
                None,
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
                            43,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            43,
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
                            Original(
                                TodoBoxColon,
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
                            46,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            46,
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
                                    34,
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    35,
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
                            48,
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
                                    36,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            52,
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
                            51,
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
                            53,
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
                                    37,
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
                            58,
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
                            57,
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
                            59,
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
                            60,
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
                            Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 16,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            61,
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
                                                                value: 16,
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
                                                        value: 36,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            62,
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
                                                                        value: 36,
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
                            63,
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
                                EntityPathError,
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    41,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            64,
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
                            65,
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
                                    43,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            66,
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
                            67,
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
                                    45,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            68,
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
                            69,
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
                            70,
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
                            71,
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
                            72,
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
                                                        value: 34,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            73,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            73,
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
                                                        value: 34,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            74,
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
                                                                        value: 34,
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
                            75,
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
                                    49,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            76,
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
                            77,
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    50,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            78,
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
                                    51,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            80,
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
                            79,
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
                            81,
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
                                    52,
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            83,
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
                            82,
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
                            84,
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
                            85,
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
                            86,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            86,
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
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        expectation_rule_idx: Some(
                            87,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            87,
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
                            88,
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
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
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
            Term(`Ref TermLiteral::EvalLifetime ConnectedComponent`),
        ],
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`Direction`),
            ),
            LocalTerm::Resolved(
                Term(`Direction`),
            ),
            LocalTerm::Resolved(
                Term(`Direction`),
            ),
            LocalTerm::Resolved(
                Term(`i32`),
            ),
        ],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 44,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 6,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
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
                        src_expr_idx: 9,
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
                        src_expr_idx: 16,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 16,
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
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 14,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 16,
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
                        src_expr_idx: 20,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
                                ),
                                src_expr_idx: 20,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 20,
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
                    UnresolvedTermEntry {
                        src_expr_idx: 22,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    5,
                                ),
                                src_expr_idx: 22,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 25,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    6,
                                ),
                                src_expr_idx: 25,
                                variant: UnspecifiedIntegerType,
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
                                    7,
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
                        src_expr_idx: 47,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    8,
                                ),
                                src_expr_idx: 47,
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
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        10,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    10,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 50,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    9,
                                ),
                                src_expr_idx: 50,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 53,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    10,
                                ),
                                src_expr_idx: 53,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 57,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    11,
                                ),
                                src_expr_idx: 57,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 57,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        14,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    14,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 65,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    12,
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
                                        16,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    16,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 72,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    13,
                                ),
                                src_expr_idx: 72,
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
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    14,
                                ),
                                src_expr_idx: 70,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 72,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        18,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    18,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 76,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    15,
                                ),
                                src_expr_idx: 76,
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
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        21,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    21,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 85,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    16,
                                ),
                                src_expr_idx: 85,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 86,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    17,
                                ),
                                src_expr_idx: 86,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 87,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    18,
                                ),
                                src_expr_idx: 87,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
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
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 90,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    20,
                                ),
                                src_expr_idx: 90,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 92,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    21,
                                ),
                                src_expr_idx: 92,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 131,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    22,
                                ),
                                src_expr_idx: 131,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 129,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    23,
                                ),
                                src_expr_idx: 129,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 129,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        30,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    30,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 127,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    24,
                                ),
                                src_expr_idx: 127,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 127,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        32,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    32,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 133,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    25,
                                ),
                                src_expr_idx: 133,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 136,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    26,
                                ),
                                src_expr_idx: 136,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 142,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    27,
                                ),
                                src_expr_idx: 142,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 145,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    28,
                                ),
                                src_expr_idx: 145,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 151,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    29,
                                ),
                                src_expr_idx: 151,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 160,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    30,
                                ),
                                src_expr_idx: 160,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 170,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    31,
                                ),
                                src_expr_idx: 170,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 167,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    32,
                                ),
                                src_expr_idx: 167,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 174,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    33,
                                ),
                                src_expr_idx: 174,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 171,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    34,
                                ),
                                src_expr_idx: 171,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 177,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    35,
                                ),
                                src_expr_idx: 177,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 176,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    36,
                                ),
                                src_expr_idx: 176,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 180,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    37,
                                ),
                                src_expr_idx: 180,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 183,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    38,
                                ),
                                src_expr_idx: 183,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 186,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    39,
                                ),
                                src_expr_idx: 186,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 187,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    40,
                                ),
                                src_expr_idx: 187,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 193,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    41,
                                ),
                                src_expr_idx: 193,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 197,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    42,
                                ),
                                src_expr_idx: 197,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 201,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    43,
                                ),
                                src_expr_idx: 201,
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
                            src_expr_idx: 0,
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
                            src_expr_idx: 6,
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
                            src_expr_idx: 9,
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
                            src_expr_idx: 14,
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
                            src_expr_idx: 20,
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
                            src_expr_idx: 25,
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
                            src_expr_idx: 47,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    11,
                                ),
                            ),
                            expectation: EqsRefMutApplication {
                                lifetime: Unresolved(
                                    UnresolvedTermIdx(
                                        9,
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
                            src_expr_idx: 50,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    12,
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
                            src_expr_idx: 53,
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
                            src_expr_idx: 57,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    15,
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
                            src_expr_idx: 58,
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
                            src_expr_idx: 65,
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
                            src_expr_idx: 70,
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
                            src_expr_idx: 72,
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
                            src_expr_idx: 76,
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
                            src_expr_idx: 77,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 13,
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
                                                                value: 13,
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
                            src_expr_idx: 81,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
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
                                                                        value: 34,
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
                            src_expr_idx: 84,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
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
                                                                        value: 34,
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
                            src_expr_idx: 86,
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
                            src_expr_idx: 87,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    25,
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
                            src_expr_idx: 88,
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
                            src_expr_idx: 90,
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
                            src_expr_idx: 92,
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
                            src_expr_idx: 98,
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
                            src_expr_idx: 99,
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
                            src_expr_idx: 102,
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
                            src_expr_idx: 100,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
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
                                                                        value: 34,
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
                            src_expr_idx: 101,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
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
                                                            value: 34,
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
                                                                        value: 34,
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
                                                                        value: 34,
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
                            src_expr_idx: 103,
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
                            src_expr_idx: 104,
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
                            src_expr_idx: 105,
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
                            src_expr_idx: 106,
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
                            src_expr_idx: 107,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 14,
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
                                                                value: 14,
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
                            src_expr_idx: 111,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
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
                                                            value: 34,
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
                                                                        value: 34,
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
                                                                        value: 34,
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
                            src_expr_idx: 112,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
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
                                                                        value: 34,
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
                            src_expr_idx: 113,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 15,
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
                                                                value: 15,
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
                            src_expr_idx: 114,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
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
                                                            value: 34,
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
                                                                        value: 34,
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
                                                                        value: 34,
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
                            src_expr_idx: 115,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
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
                                                            value: 34,
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
                                                                        value: 34,
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
                                                                        value: 34,
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
                            src_expr_idx: 116,
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
                            src_expr_idx: 129,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    31,
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
                            src_expr_idx: 127,
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
                            src_expr_idx: 131,
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
                            src_expr_idx: 132,
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
                            src_expr_idx: 133,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    34,
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
                            src_expr_idx: 139,
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
                            src_expr_idx: 136,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    35,
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
                            src_expr_idx: 143,
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
                            src_expr_idx: 142,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    36,
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
                            src_expr_idx: 144,
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
                            src_expr_idx: 148,
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
                            src_expr_idx: 145,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    37,
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
                            src_expr_idx: 149,
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
                            src_expr_idx: 152,
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
                            src_expr_idx: 151,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    38,
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
                            src_expr_idx: 156,
                            expectee: Resolved(
                                ReducedTerm(
                                    Ritchie(
                                        TermRitchie(
                                            Id {
                                                value: 16,
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
                                                                value: 16,
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
                            src_expr_idx: 159,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 36,
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
                                                                        value: 36,
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
                            src_expr_idx: 160,
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
                            src_expr_idx: 167,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    41,
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
                            src_expr_idx: 170,
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
                            src_expr_idx: 171,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    43,
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
                            src_expr_idx: 174,
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
                            src_expr_idx: 176,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    45,
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
                            src_expr_idx: 177,
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
                            src_expr_idx: 180,
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
                            src_expr_idx: 182,
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
                            src_expr_idx: 183,
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
                            src_expr_idx: 184,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
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
                                        48,
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
                            src_expr_idx: 185,
                            expectee: Resolved(
                                ReducedTerm(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 34,
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
                                                                        value: 34,
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
                            src_expr_idx: 186,
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
                            src_expr_idx: 187,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    49,
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
                            src_expr_idx: 190,
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
                            src_expr_idx: 193,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    50,
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
                            src_expr_idx: 198,
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
                            src_expr_idx: 197,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    51,
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
                            src_expr_idx: 199,
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
                            src_expr_idx: 202,
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
                            src_expr_idx: 201,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    52,
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
                            src_expr_idx: 203,
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
                            src_expr_idx: 204,
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
                            src_expr_idx: 208,
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
                            src_expr_idx: 209,
                            expectee: Resolved(
                                ReducedTerm(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 9,
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
                                                        value: 9,
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
                            src_expr_idx: 214,
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
            Term(`List RawContour`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::raw_contour`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        },
                    },
                    ident: `line_segment_sketch`,
                },
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
                Some(
                    ExprTypeInfo {
                        ty_result: Err(
                            Derived(
                                EntityPathError,
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
                        src_expr_idx: 2,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 2,
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
                            src_expr_idx: 2,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            Term(`LineSegmentSketch`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::raw_contour`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        },
                    },
                    ident: `bounding_box`,
                },
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
                None,
                None,
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
                            0,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Unresolved(
                                                UnresolvedTermIdx(
                                                    0,
                                                ),
                                            ),
                                            destination: Unresolved(
                                                UnresolvedTermIdx(
                                                    0,
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
                                    2,
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
                            5,
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
                                EntityTypeError,
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
                            8,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Err(
                                    Derived(
                                        Duplication(
                                            8,
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
                            9,
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
                next: 7,
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
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 2,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
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
                        src_expr_idx: 3,
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
                        src_expr_idx: 20,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 20,
                                variant: ImplicitType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 20,
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
                        src_expr_idx: 26,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 26,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 32,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
                                ),
                                src_expr_idx: 32,
                                variant: ExprEvalLifetime,
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
                                    5,
                                ),
                                src_expr_idx: 38,
                                variant: ExprEvalLifetime,
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
                                    6,
                                ),
                                src_expr_idx: 44,
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
                            src_expr_idx: 2,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    1,
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
                                            expectee: Unresolved(
                                                UnresolvedTermIdx(
                                                    0,
                                                ),
                                            ),
                                            destination: Unresolved(
                                                UnresolvedTermIdx(
                                                    0,
                                                ),
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
                            src_expr_idx: 32,
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
                            src_expr_idx: 44,
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
                            src_expr_idx: 46,
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
            Term(`BoundingBox`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::raw_contour`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        },
                    },
                    ident: `relative_bounding_box`,
                },
            ),
        ),
        expr_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
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
                            0,
                        ),
                        resolve_progress: Expected(
                            Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        ExpectImplicitlyConvertibleResolvedOk {
                                            implicit_conversion: None,
                                            expectee: Unresolved(
                                                UnresolvedTermIdx(
                                                    0,
                                                ),
                                            ),
                                            destination: Unresolved(
                                                UnresolvedTermIdx(
                                                    0,
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
                                    2,
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
            ],
        },
        inherited_symbol_tys: [],
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
                                variant: ImplicitType,
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
                            src_expr_idx: 3,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    1,
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
                                            expectee: Unresolved(
                                                UnresolvedTermIdx(
                                                    0,
                                                ),
                                            ),
                                            destination: Unresolved(
                                                UnresolvedTermIdx(
                                                    0,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            Term(`RelativeBoundingBox`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::raw_contour`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        },
                    },
                    ident: `contour_len`,
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
                                    4,
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
                            4,
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
                            5,
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
                None,
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    9,
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
                None,
                None,
                Some(
                    ExprTypeInfo {
                        ty_result: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    12,
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
                                            expectee: Unresolved(
                                                UnresolvedTermIdx(
                                                    11,
                                                ),
                                            ),
                                            destination: Unresolved(
                                                UnresolvedTermIdx(
                                                    11,
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
                                    13,
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
                                variant: UnspecifiedFloatType,
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
                        src_expr_idx: 13,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 13,
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
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 11,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 13,
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
                        src_expr_idx: 17,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
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
                        src_expr_idx: 17,
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
                        src_expr_idx: 42,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    6,
                                ),
                                src_expr_idx: 42,
                                variant: ImplicitType,
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
                    UnresolvedTermEntry {
                        src_expr_idx: 42,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
                                },
                            ),
                            arguments: [
                                Unresolved(
                                    UnresolvedTermIdx(
                                        8,
                                    ),
                                ),
                            ],
                        },
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    8,
                                ),
                            ],
                        },
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 46,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    8,
                                ),
                                src_expr_idx: 46,
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
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    9,
                                ),
                                src_expr_idx: 45,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [
                                UnresolvedTermIdx(
                                    11,
                                ),
                            ],
                        },
                        resolve_progress: PartiallyResolved(
                            UnresolvedTermIdx(
                                11,
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 46,
                        unresolved_term: TypeApplication {
                            ty: TypePath(
                                Id {
                                    value: 23,
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
                        resolve_progress: Unresolved,
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 63,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    10,
                                ),
                                src_expr_idx: 63,
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
                            src_expr_idx: 13,
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
                            src_expr_idx: 40,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    9,
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
                            src_expr_idx: 42,
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
                            src_expr_idx: 45,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ),
                            expectation: ImplicitlyConversion {
                                destination: Unresolved(
                                    UnresolvedTermIdx(
                                        11,
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
                                                    11,
                                                ),
                                            ),
                                            destination: Unresolved(
                                                UnresolvedTermIdx(
                                                    11,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationEntry {
                            src_expr_idx: 46,
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
                            src_expr_idx: 63,
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
                            src_expr_idx: 65,
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
                        module_path: `mnist_classifier::raw_contour`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        },
                    },
                    ident: `displacement`,
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
                None,
                None,
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
                                    3,
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
            ],
        },
        inherited_symbol_tys: [
            Term(`i32`),
            Term(`i32`),
        ],
        current_symbol_tys: [],
        local_term_table: LocalTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 2,
            },
            unresolved_terms: UnresolvedTerms {
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 8,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
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
                        src_expr_idx: 14,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
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
                ],
                first_unresolved_term: 0,
            },
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationEntry {
                            src_expr_idx: 5,
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
                            src_expr_idx: 11,
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
                            src_expr_idx: 14,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            Term(`Vector2d`),
        ),
        self_ty: None,
    },
]