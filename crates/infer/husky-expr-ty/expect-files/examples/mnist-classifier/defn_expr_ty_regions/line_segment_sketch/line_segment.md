[
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_id: ImplId {
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                        impl_kind: ImplKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        },
                        disambiguator: 0,
                    },
                    ident: `displacement`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                SelfTypeNotInferredForSelfValue,
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
                                SelfTypeNotInferredForSelfValue,
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
                                MethodOwnerTypeNotInferred,
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
        current_symbol_tys: [],
        local_term_region: LocalTermRegion {
            unresolved_terms: UnresolvedTerms {
                implicit_symbol_registry: ImplicitSymbolRegistry {
                    next: 0,
                },
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
        return_ty: Some(
            Term(`TypeOntology(mnist_classifier::geom2d::Vector2d)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_id: ImplId {
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                        impl_kind: ImplKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        },
                        disambiguator: 0,
                    },
                    ident: `dist_to_point`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                SelfTypeNotInferredForSelfValue,
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
                ty_result: Ok(
                    (
                        Trivial,
                        Err(
                            Derived(
                                MethodOwnerTypeNotInferred,
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
                                SelfTypeNotInferredForSelfValue,
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
                    1,
                ),
                resolve_progress: Expected(
                    Unresolved,
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
                            Derived(
                                MethodOwnerTypeNotInferred,
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
                        Err(
                            Derived(
                                MethodOwnerTypeNotInferred,
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
                                ExpectImplicitlyConvertibleOutcome {
                                    implicit_conversion: None,
                                    expectee: Resolved(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    destination: Resolved(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 1,
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
                                MethodOwnerTypeNotInferred,
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
                                SelfTypeNotInferredForSelfValue,
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
                    5,
                ),
                resolve_progress: Expected(
                    Unresolved,
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
                            Derived(
                                MethodOwnerTypeNotInferred,
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
                                MethodOwnerTypeNotInferred,
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
                            ImplicitlyConvertible(
                                ExpectImplicitlyConvertibleOutcome {
                                    implicit_conversion: None,
                                    expectee: Resolved(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    destination: Resolved(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 1,
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
                                MethodOwnerTypeNotInferred,
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
                        Err(
                            Derived(
                                MethodOwnerTypeNotInferred,
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
                                MethodOwnerTypeNotInferred,
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
                                MethodOwnerTypeNotInferred,
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
            ],
        },
        inherited_symbol_tys: [
            Term(`TypeOntology(mnist_classifier::geom2d::Point2d)`),
        ],
        current_symbol_tys: [],
        local_term_region: LocalTermRegion {
            unresolved_terms: UnresolvedTerms {
                implicit_symbol_registry: ImplicitSymbolRegistry {
                    next: 5,
                },
                arena: [
                    UnresolvedTermEntry {
                        src_expr_idx: 3,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    0,
                                ),
                                src_expr_idx: 3,
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
                                    1,
                                ),
                                src_expr_idx: 9,
                                variant: ExprEvalLifetime,
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
                                variant: UnspecifiedFloatType,
                            },
                        ),
                        implicit_symbol_dependencies: VecSet {
                            data: [],
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
                        src_expr_idx: 26,
                        unresolved_term: ImplicitSymbol(
                            ImplicitSymbol {
                                idx: ImplicitSymbolIdx(
                                    4,
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
                            src_expr_idx: 3,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 3,
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
                        LocalTermExpectationEntry {
                            src_expr_idx: 6,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 47,
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
                        LocalTermExpectationEntry {
                            src_expr_idx: 9,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 3,
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
                        LocalTermExpectationEntry {
                            src_expr_idx: 13,
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
                        LocalTermExpectationEntry {
                            src_expr_idx: 14,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 1,
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
                                                        value: 1,
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
                                        ExpectImplicitlyConvertibleOutcome {
                                            implicit_conversion: None,
                                            expectee: Resolved(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 1,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            destination: Resolved(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 1,
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
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 47,
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
                        LocalTermExpectationEntry {
                            src_expr_idx: 22,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 3,
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
                        LocalTermExpectationEntry {
                            src_expr_idx: 26,
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
                        LocalTermExpectationEntry {
                            src_expr_idx: 27,
                            expectee: Resolved(
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 1,
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
                                                        value: 1,
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
                                        ExpectImplicitlyConvertibleOutcome {
                                            implicit_conversion: None,
                                            expectee: Resolved(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 1,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            destination: Resolved(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 1,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            Term(`TypeOntology(core::num::f32)`),
        ),
        self_ty: None,
    },
]