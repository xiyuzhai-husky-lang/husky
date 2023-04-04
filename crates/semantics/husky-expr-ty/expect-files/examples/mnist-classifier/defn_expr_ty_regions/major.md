[
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`mnist_classifier::major::connected_components`, `Val`),
                    ),
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
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                BlockTypeError,
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
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
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
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
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
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
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
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
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
                        Err(
                            Derived(
                                CurrentSymbolTypeError,
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
                        Err(
                            Derived(
                                CurrentSymbolTypeError,
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
                                EntityPath(
                                    TypeOntology(
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
                expectation_rule_idx: Some(
                    2,
                ),
                resolve_progress: Expected(
                    Resolved(
                        Ok(
                            ImplicitlyConvertible(
                                None,
                            ),
                        ),
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                CurrentSymbolTypeError,
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
                        Err(
                            Derived(
                                CurrentSymbolTypeError,
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
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 33,
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
                                None,
                            ),
                        ),
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                CurrentSymbolTypeError,
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
                        Err(
                            Derived(
                                CurrentSymbolTypeError,
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
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 33,
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
                                None,
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
                                    SignatureError,
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
                        Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
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
                                EntityPath(
                                    TypeOntology(
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
                    5,
                ),
                resolve_progress: Expected(
                    Unresolved,
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
                    next: 4,
                },
                data: [
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
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 1,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 1,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 15,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 15,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    2,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 18,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 18,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    3,
                                ),
                            ),
                        ),
                    },
                ],
                first_unresolved_term: 0,
            },
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationRule {
                            src_expr_idx: 0,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 1,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 12,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 31,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    destination: Resolved(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        None,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 15,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 33,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    destination: Resolved(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        None,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 18,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 33,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    destination: Resolved(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        None,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 22,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
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
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
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
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
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
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                CurrentSymbolTypeError,
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
                        Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
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
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 33,
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
                                None,
                            ),
                        ),
                    ),
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                CurrentSymbolTypeError,
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
                        Err(
                            Derived(
                                BinaryOperationLeftOperandTypeNotInferred,
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
                                EntityPath(
                                    TypeOntology(
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
                    2,
                ),
                resolve_progress: Expected(
                    Unresolved,
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
                    next: 2,
                },
                data: [
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
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 10,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 10,
                                variant: ExprEvalLifetime,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                        ),
                    },
                ],
                first_unresolved_term: 0,
            },
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationRule {
                            src_expr_idx: 0,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 10,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 33,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    destination: Resolved(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        None,
                                    ),
                                ),
                            ),
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 15,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
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
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                    ),
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
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
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
                        Err(
                            Derived(
                                BlockTypeError,
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
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                    ),
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
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                FieldOwnerTypeNotInferred,
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
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
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                BlockTypeError,
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
                    next: 1,
                },
                data: [
                    UnresolvedTermEntry {
                        src_expr_idx: 2,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 2,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                        ),
                    },
                ],
                first_unresolved_term: 0,
            },
            expectations: LocalTermExpectations {
                arena: Arena {
                    data: [
                        LocalTermExpectationRule {
                            src_expr_idx: 2,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
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
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Resolved(
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
                ),
                expectation_rule_idx: Some(
                    0,
                ),
                resolve_progress: Expected(
                    Unresolved,
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Original(
                                NoSuchField,
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
                        Err(
                            Derived(
                                BlockTypeError,
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
                    data: [
                        LocalTermExpectationRule {
                            src_expr_idx: 0,
                            expectee: Resolved(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
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
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Resolved(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 12,
                                        },
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
                    Unresolved,
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Original(
                                NoSuchField,
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
                        Err(
                            Derived(
                                BlockTypeError,
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
                    data: [
                        LocalTermExpectationRule {
                            src_expr_idx: 0,
                            expectee: Resolved(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 12,
                                        },
                                    ),
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
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