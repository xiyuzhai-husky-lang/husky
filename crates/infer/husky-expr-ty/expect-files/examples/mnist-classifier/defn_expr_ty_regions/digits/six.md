[
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                        List(
                            NewList,
                        ),
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
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 3,
                        unresolved_term: TypeOntology {
                            path: TypePath(
                                Id {
                                    value: 57,
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
                            src_expr_idx: 3,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    1,
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
                        FormPath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                        List(
                            NewList,
                        ),
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
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 4,
                        unresolved_term: TypeOntology {
                            path: TypePath(
                                Id {
                                    value: 57,
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
                            src_expr_idx: 4,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    1,
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
                        FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                            value: 10,
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
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
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
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                FieldTypeTermError(
                                    RawTypeError(
                                        Derived(
                                            SignatureError,
                                        ),
                                    ),
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
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
                ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 72,
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
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                FieldTypeTermError(
                                    RawTypeError(
                                        Derived(
                                            SignatureError,
                                        ),
                                    ),
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
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 72,
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
                    Unresolved,
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                FieldTypeTermError(
                                    RawTypeError(
                                        Derived(
                                            SignatureError,
                                        ),
                                    ),
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
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
                    8,
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
                            Original(
                                TodoSuffix,
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
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
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
                    Unresolved,
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                FieldTypeTermError(
                                    RawTypeError(
                                        Derived(
                                            SignatureError,
                                        ),
                                    ),
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    2,
                                ),
                            ),
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
                ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
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
                        MethodOwnerTypeNotInferred,
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
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
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
                    Unresolved,
                ),
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                FieldTypeTermError(
                                    RawTypeError(
                                        Derived(
                                            SignatureError,
                                        ),
                                    ),
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
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
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
                        Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    3,
                                ),
                            ),
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
                    14,
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
                    15,
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
                        Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    4,
                                ),
                            ),
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    4,
                                ),
                            ),
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
                    18,
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
                        ExprError,
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
                            Original(
                                TodoSuffix,
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
                        EntityPathError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 83,
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
                        MethodOwnerTypeNotInferred,
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
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Original(
                                TodoSuffix,
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
                                    5,
                                ),
                            ),
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
                    21,
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
                        EntityPathError,
                    ),
                ),
                expectation_rule_idx: None,
                resolve_progress: Unresolved,
            },
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        EntityPathError,
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
                            Original(
                                TodoSuffix,
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
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
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
                        Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    6,
                                ),
                            ),
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
                    24,
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
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
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
                        Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    7,
                                ),
                            ),
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
                    27,
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
                                    8,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    28,
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
                                FieldOwnerTypeNotInferred,
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
                ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    9,
                                ),
                            ),
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
                    30,
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
                                    10,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    31,
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
                    32,
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
                                    11,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    33,
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
                                FieldOwnerTypeNotInferred,
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
                ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ),
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
                    35,
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
                        Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    13,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    36,
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
                    37,
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
                        EntityPathError,
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
        current_symbol_tys: [
            LocalTerm::Resolved(
                Term(`TypeOntology(core::option::Option) TypeOntology(mnist::MnistLabel)`),
            ),
        ],
        local_term_region: LocalTermRegion {
            unresolved_terms: UnresolvedTerms {
                implicit_symbol_registry: ImplicitSymbolRegistry {
                    next: 14,
                },
                data: [
                    UnresolvedTermEntry {
                        src_expr_idx: 4,
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
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 17,
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
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 31,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 31,
                                variant: UnspecifiedIntegerType,
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
                        src_expr_idx: 44,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 44,
                                variant: UnspecifiedFloatType,
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
                    UnresolvedTermEntry {
                        src_expr_idx: 48,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
                                ),
                                src_expr_idx: 48,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    4,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 79,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    5,
                                ),
                                src_expr_idx: 79,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    5,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 92,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    6,
                                ),
                                src_expr_idx: 92,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    6,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 96,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    7,
                                ),
                                src_expr_idx: 96,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    7,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 100,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    8,
                                ),
                                src_expr_idx: 100,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    8,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 104,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    9,
                                ),
                                src_expr_idx: 104,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    9,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 108,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    10,
                                ),
                                src_expr_idx: 108,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    10,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 113,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    11,
                                ),
                                src_expr_idx: 113,
                                variant: UnspecifiedIntegerType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    11,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 117,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    12,
                                ),
                                src_expr_idx: 117,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 120,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    13,
                                ),
                                src_expr_idx: 120,
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
                        },
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    13,
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
                            expectee: Resolved(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 1,
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
                            src_expr_idx: 2,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 4,
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
                        LocalTermExpectationRule {
                            src_expr_idx: 7,
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
                            src_expr_idx: 8,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 72,
                                            },
                                        ),
                                    ),
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
                                                value: 72,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 17,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 19,
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
                            src_expr_idx: 29,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 31,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    2,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 40,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 42,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 44,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    3,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 45,
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
                            src_expr_idx: 47,
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
                            src_expr_idx: 48,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    4,
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 50,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    4,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 51,
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
                            src_expr_idx: 61,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 83,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 79,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    5,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 80,
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
                            src_expr_idx: 90,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 92,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    6,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 93,
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
                            src_expr_idx: 94,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 96,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    7,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 97,
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
                            src_expr_idx: 100,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    8,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 104,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    9,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 105,
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
                            src_expr_idx: 108,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    10,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 110,
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
                            src_expr_idx: 113,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    11,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 117,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    12,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 118,
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
                            src_expr_idx: 120,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    13,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 121,
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
                        FormPath(`mnist_classifier::digits::six::upmost`, `Fn`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                InheritedSymbolTypeError,
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
                    next: 1,
                },
                data: [
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
                            src_expr_idx: 4,
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
                        LocalTermExpectationRule {
                            src_expr_idx: 5,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            Term(`TypeOntology(core::option::Option) TypeOntology(core::num::f32)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FormPath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                InheritedSymbolTypeError,
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
                    1,
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
                        Err(
                            Derived(
                                BracketedItemTypeError,
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
                    3,
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
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                InheritedSymbolTypeError,
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
                ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
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
                                    2,
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
                    6,
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
                                InheritedSymbolTypeError,
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
                                InheritedSymbolTypeError,
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
                        MethodOwnerTypeNotInferred,
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
                            Unresolved(
                                UnresolvedTermIdx(
                                    3,
                                ),
                            ),
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
                    8,
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
                                InheritedSymbolTypeError,
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
                ty_result: Err(
                    Derived(
                        PrefixOperandTypeNotInferred,
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
                    next: 4,
                },
                data: [
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
                        resolve_progress: Ok(
                            Unresolved(
                                UnresolvedTermIdx(
                                    0,
                                ),
                            ),
                        ),
                    },
                    UnresolvedTermEntry {
                        src_expr_idx: 14,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    1,
                                ),
                                src_expr_idx: 14,
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
                        src_expr_idx: 19,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    2,
                                ),
                                src_expr_idx: 19,
                                variant: UnspecifiedFloatType,
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
                        src_expr_idx: 29,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    3,
                                ),
                                src_expr_idx: 29,
                                variant: UnspecifiedFloatType,
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
                            src_expr_idx: 3,
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
                            src_expr_idx: 5,
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
                        LocalTermExpectationRule {
                            src_expr_idx: 6,
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
                            src_expr_idx: 14,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    1,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 15,
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
                            src_expr_idx: 19,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    2,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 20,
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
                            src_expr_idx: 29,
                            expectee: Unresolved(
                                UnresolvedTermIdx(
                                    3,
                                ),
                            ),
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        LocalTermExpectationRule {
                            src_expr_idx: 30,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            Term(`TypeOntology(core::option::Option) TypeOntology(core::num::f32)`),
        ),
        self_ty: None,
    },
]