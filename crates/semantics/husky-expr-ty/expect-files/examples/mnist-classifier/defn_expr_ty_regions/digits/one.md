[
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        List(
                            NewList,
                        ),
                        Ok(
                            Hollow(
                                HollowTerm(
                                    1,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    5,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 42,
                                    },
                                ),
                                refined_path: Left(
                                    List,
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
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
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 51,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    5,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Err(
                            Derived(
                                UnableToInferReturnTypeForUnveiling,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        List(
                            NewList,
                        ),
                        Ok(
                            Hollow(
                                HollowTerm(
                                    8,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    9,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    4,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    5,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    6,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 51,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    7,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    15,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    8,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Err(
                            Derived(
                                UnableToInferReturnTypeForUnveiling,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    17,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    9,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    10,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    18,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    11,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    12,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 97,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    13,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    19,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    14,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    15,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    20,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    16,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    17,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    21,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    18,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    22,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    19,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    23,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    20,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    21,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    24,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    22,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    23,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    24,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 51,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    25,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    30,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    26,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Err(
                            Derived(
                                UnableToInferReturnTypeForUnveiling,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    32,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    27,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    32,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    28,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    32,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    29,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    32,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    30,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    31,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 97,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    32,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    33,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    33,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    34,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    35,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    41,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    36,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 51,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    37,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    39,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    38,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Err(
                            Derived(
                                UnableToInferReturnTypeForUnveiling,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    39,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    49,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    40,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    50,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    41,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 51,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    42,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    47,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    43,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Err(
                            Derived(
                                UnableToInferReturnTypeForUnveiling,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    44,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    51,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    45,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    46,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    52,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    47,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    53,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    48,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    49,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    50,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 51,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    51,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    59,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    52,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Err(
                            Derived(
                                UnableToInferReturnTypeForUnveiling,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    53,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 51,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    54,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    66,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    55,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Err(
                            Derived(
                                UnableToInferReturnTypeForUnveiling,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    56,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    57,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    75,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    58,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    76,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    59,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    77,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    60,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 51,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    61,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    73,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    62,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Err(
                            Derived(
                                UnableToInferReturnTypeForUnveiling,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    78,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    63,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    64,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    79,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    65,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    66,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 51,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    67,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    85,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    68,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Err(
                            Derived(
                                UnableToInferReturnTypeForUnveiling,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    87,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    69,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    70,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    SignatureError,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    88,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    71,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    88,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    72,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    73,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    89,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    74,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    75,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 97,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    76,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 97,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    77,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
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
                ],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
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
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    0,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Curry {
                                curry_kind: Implicit,
                                variance: Independent,
                                parameter_variable: Some(
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                parameter_ty: Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        2,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    0,
                                ),
                                hole_kind: Any,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            4,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        5,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    9,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 42,
                                    },
                                ),
                                refined_path: Left(
                                    List,
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            7,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    13,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    6,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            10,
                                        ),
                                    ),
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        11,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Curry {
                                curry_kind: Implicit,
                                variance: Independent,
                                parameter_variable: Some(
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                parameter_ty: Hollow(
                                    HollowTerm(
                                        10,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        12,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    6,
                                ),
                                hole_kind: Any,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            10,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            14,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        15,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    23,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    27,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    32,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    35,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    39,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    43,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    47,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    53,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    24,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            25,
                                        ),
                                    ),
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        26,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Curry {
                                curry_kind: Implicit,
                                variance: Independent,
                                parameter_variable: Some(
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                parameter_ty: Hollow(
                                    HollowTerm(
                                        25,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        27,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    24,
                                ),
                                hole_kind: Any,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            25,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            29,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        30,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    63,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [
                                    ImplicitlyConvertibleFrom {
                                        target: Hollow(
                                            HollowTerm(
                                                32,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    81,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    35,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            34,
                                        ),
                                    ),
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        35,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Curry {
                                curry_kind: Implicit,
                                variance: Independent,
                                parameter_variable: Some(
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                parameter_ty: Hollow(
                                    HollowTerm(
                                        34,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        36,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    35,
                                ),
                                hole_kind: Any,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            34,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            38,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        39,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    85,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    39,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            42,
                                        ),
                                    ),
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        43,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Curry {
                                curry_kind: Implicit,
                                variance: Independent,
                                parameter_variable: Some(
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                parameter_ty: Hollow(
                                    HollowTerm(
                                        42,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        44,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    39,
                                ),
                                hole_kind: Any,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            42,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            46,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        47,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    99,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    104,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    120,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    127,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    134,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    50,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            54,
                                        ),
                                    ),
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        55,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Curry {
                                curry_kind: Implicit,
                                variance: Independent,
                                parameter_variable: Some(
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                parameter_ty: Hollow(
                                    HollowTerm(
                                        54,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        56,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    50,
                                ),
                                hole_kind: Any,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            54,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            58,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        59,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    53,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            61,
                                        ),
                                    ),
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        62,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Curry {
                                curry_kind: Implicit,
                                variance: Independent,
                                parameter_variable: Some(
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                parameter_ty: Hollow(
                                    HollowTerm(
                                        61,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        63,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    53,
                                ),
                                hole_kind: Any,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            61,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            65,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        66,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    57,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            68,
                                        ),
                                    ),
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        69,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Curry {
                                curry_kind: Implicit,
                                variance: Independent,
                                parameter_variable: Some(
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                parameter_ty: Hollow(
                                    HollowTerm(
                                        68,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        70,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    57,
                                ),
                                hole_kind: Any,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            68,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            72,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        73,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    163,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    168,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    173,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    181,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    196,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    66,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            80,
                                        ),
                                    ),
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        81,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Curry {
                                curry_kind: Implicit,
                                variance: Independent,
                                parameter_variable: Some(
                                    Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                parameter_ty: Hollow(
                                    HollowTerm(
                                        80,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        82,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    66,
                                ),
                                hole_kind: Any,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 96,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            80,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            84,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Variadic(
                                        FluffyTermRitchieVariadicParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 61,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Word(
                                                    Id {
                                                        value: 444,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            default: Some(
                                                Literal(
                                                    I32(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: Hollow(
                                    HollowTerm(
                                        85,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    210,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    222,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    227,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        EqsRitchieCallType(
                                            ExpectEqsRitchieTypeOutcome {
                                                ritchie_kind: FnType,
                                                implicit_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                0,
                                                            ),
                                                        ),
                                                    },
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 2,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                4,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                parameter_contracted_tys: [
                                                    Variadic(
                                                        FluffyTermRitchieVariadicParameter {
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 61,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 444,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 51,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            default: Some(
                                                                Literal(
                                                                    I32(
                                                                        5,
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ],
                                                return_ty: Hollow(
                                                    HollowTerm(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 51,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        5,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        8,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 13,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        9,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 14,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 15,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        EqsRitchieCallType(
                                            ExpectEqsRitchieTypeOutcome {
                                                ritchie_kind: FnType,
                                                implicit_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                10,
                                                            ),
                                                        ),
                                                    },
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 2,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                14,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                parameter_contracted_tys: [
                                                    Variadic(
                                                        FluffyTermRitchieVariadicParameter {
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 61,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 444,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 51,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            default: Some(
                                                                Literal(
                                                                    I32(
                                                                        5,
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ],
                                                return_ty: Hollow(
                                                    HollowTerm(
                                                        15,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 51,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 18,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 19,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        15,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 23,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        17,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 24,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 27,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        18,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 28,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 29,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 97,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 14,
                                src: ExpectationSource {
                                    expr_idx: 32,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        19,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 33,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 35,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        20,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 36,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 39,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        21,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 19,
                                src: ExpectationSource {
                                    expr_idx: 43,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        22,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 47,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        23,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 50,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 22,
                                src: ExpectationSource {
                                    expr_idx: 53,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        24,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 23,
                                src: ExpectationSource {
                                    expr_idx: 54,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 24,
                                src: ExpectationSource {
                                    expr_idx: 55,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        EqsRitchieCallType(
                                            ExpectEqsRitchieTypeOutcome {
                                                ritchie_kind: FnType,
                                                implicit_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                25,
                                                            ),
                                                        ),
                                                    },
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 2,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                29,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                parameter_contracted_tys: [
                                                    Variadic(
                                                        FluffyTermRitchieVariadicParameter {
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 61,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 444,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 51,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            default: Some(
                                                                Literal(
                                                                    I32(
                                                                        5,
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ],
                                                return_ty: Hollow(
                                                    HollowTerm(
                                                        30,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 51,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 25,
                                src: ExpectationSource {
                                    expr_idx: 59,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 26,
                                src: ExpectationSource {
                                    expr_idx: 60,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        30,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 27,
                                src: ExpectationSource {
                                    expr_idx: 63,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        32,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 28,
                                src: ExpectationSource {
                                    expr_idx: 66,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        32,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 29,
                                src: ExpectationSource {
                                    expr_idx: 68,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        32,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: Hollow(
                                        HollowTerm(
                                            32,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 30,
                                src: ExpectationSource {
                                    expr_idx: 69,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        32,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 31,
                                src: ExpectationSource {
                                    expr_idx: 70,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 32,
                                src: ExpectationSource {
                                    expr_idx: 71,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 97,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 33,
                                src: ExpectationSource {
                                    expr_idx: 81,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        33,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 34,
                                src: ExpectationSource {
                                    expr_idx: 82,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 35,
                                src: ExpectationSource {
                                    expr_idx: 83,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        EqsRitchieCallType(
                                            ExpectEqsRitchieTypeOutcome {
                                                ritchie_kind: FnType,
                                                implicit_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                34,
                                                            ),
                                                        ),
                                                    },
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 2,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                38,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                parameter_contracted_tys: [
                                                    Variadic(
                                                        FluffyTermRitchieVariadicParameter {
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 61,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 444,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 51,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            default: Some(
                                                                Literal(
                                                                    I32(
                                                                        5,
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ],
                                                return_ty: Hollow(
                                                    HollowTerm(
                                                        39,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 36,
                                src: ExpectationSource {
                                    expr_idx: 85,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        41,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 51,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 37,
                                src: ExpectationSource {
                                    expr_idx: 93,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 38,
                                src: ExpectationSource {
                                    expr_idx: 94,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        39,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 39,
                                src: ExpectationSource {
                                    expr_idx: 96,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        EqsRitchieCallType(
                                            ExpectEqsRitchieTypeOutcome {
                                                ritchie_kind: FnType,
                                                implicit_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                42,
                                                            ),
                                                        ),
                                                    },
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 2,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                46,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                parameter_contracted_tys: [
                                                    Variadic(
                                                        FluffyTermRitchieVariadicParameter {
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 61,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 444,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 51,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            default: Some(
                                                                Literal(
                                                                    I32(
                                                                        5,
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ],
                                                return_ty: Hollow(
                                                    HollowTerm(
                                                        47,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 40,
                                src: ExpectationSource {
                                    expr_idx: 99,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        49,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 41,
                                src: ExpectationSource {
                                    expr_idx: 104,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        50,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 51,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 42,
                                src: ExpectationSource {
                                    expr_idx: 108,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 43,
                                src: ExpectationSource {
                                    expr_idx: 109,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        47,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 44,
                                src: ExpectationSource {
                                    expr_idx: 112,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 45,
                                src: ExpectationSource {
                                    expr_idx: 120,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        51,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 46,
                                src: ExpectationSource {
                                    expr_idx: 121,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 47,
                                src: ExpectationSource {
                                    expr_idx: 127,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        52,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 48,
                                src: ExpectationSource {
                                    expr_idx: 134,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        53,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 49,
                                src: ExpectationSource {
                                    expr_idx: 135,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 50,
                                src: ExpectationSource {
                                    expr_idx: 136,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        EqsRitchieCallType(
                                            ExpectEqsRitchieTypeOutcome {
                                                ritchie_kind: FnType,
                                                implicit_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                54,
                                                            ),
                                                        ),
                                                    },
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 2,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                58,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                parameter_contracted_tys: [
                                                    Variadic(
                                                        FluffyTermRitchieVariadicParameter {
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 61,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 444,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 51,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            default: Some(
                                                                Literal(
                                                                    I32(
                                                                        5,
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ],
                                                return_ty: Hollow(
                                                    HollowTerm(
                                                        59,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 51,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 51,
                                src: ExpectationSource {
                                    expr_idx: 144,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 52,
                                src: ExpectationSource {
                                    expr_idx: 145,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        59,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 53,
                                src: ExpectationSource {
                                    expr_idx: 147,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        EqsRitchieCallType(
                                            ExpectEqsRitchieTypeOutcome {
                                                ritchie_kind: FnType,
                                                implicit_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                61,
                                                            ),
                                                        ),
                                                    },
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 2,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                65,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                parameter_contracted_tys: [
                                                    Variadic(
                                                        FluffyTermRitchieVariadicParameter {
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 61,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 444,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 51,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            default: Some(
                                                                Literal(
                                                                    I32(
                                                                        5,
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ],
                                                return_ty: Hollow(
                                                    HollowTerm(
                                                        66,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 51,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 54,
                                src: ExpectationSource {
                                    expr_idx: 155,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 55,
                                src: ExpectationSource {
                                    expr_idx: 156,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        66,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 56,
                                src: ExpectationSource {
                                    expr_idx: 159,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 57,
                                src: ExpectationSource {
                                    expr_idx: 160,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        EqsRitchieCallType(
                                            ExpectEqsRitchieTypeOutcome {
                                                ritchie_kind: FnType,
                                                implicit_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                68,
                                                            ),
                                                        ),
                                                    },
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 2,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                72,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                parameter_contracted_tys: [
                                                    Variadic(
                                                        FluffyTermRitchieVariadicParameter {
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 61,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 444,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 51,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            default: Some(
                                                                Literal(
                                                                    I32(
                                                                        5,
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ],
                                                return_ty: Hollow(
                                                    HollowTerm(
                                                        73,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 58,
                                src: ExpectationSource {
                                    expr_idx: 163,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        75,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 59,
                                src: ExpectationSource {
                                    expr_idx: 168,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        76,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 60,
                                src: ExpectationSource {
                                    expr_idx: 173,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        77,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 51,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 61,
                                src: ExpectationSource {
                                    expr_idx: 177,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 62,
                                src: ExpectationSource {
                                    expr_idx: 178,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        73,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 63,
                                src: ExpectationSource {
                                    expr_idx: 181,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        78,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 64,
                                src: ExpectationSource {
                                    expr_idx: 182,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 65,
                                src: ExpectationSource {
                                    expr_idx: 196,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        79,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 66,
                                src: ExpectationSource {
                                    expr_idx: 201,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        EqsRitchieCallType(
                                            ExpectEqsRitchieTypeOutcome {
                                                ritchie_kind: FnType,
                                                implicit_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                80,
                                                            ),
                                                        ),
                                                    },
                                                    ImplicitParameterSubstitution {
                                                        variable: Variable(
                                                            EtherealTermVariable(
                                                                Id {
                                                                    value: 2,
                                                                },
                                                            ),
                                                        ),
                                                        substitute: Hollow(
                                                            HollowTerm(
                                                                84,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                parameter_contracted_tys: [
                                                    Variadic(
                                                        FluffyTermRitchieVariadicParameter {
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 61,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Word(
                                                                    Id {
                                                                        value: 444,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 51,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            default: Some(
                                                                Literal(
                                                                    I32(
                                                                        5,
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ],
                                                return_ty: Hollow(
                                                    HollowTerm(
                                                        85,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 51,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 67,
                                src: ExpectationSource {
                                    expr_idx: 206,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 68,
                                src: ExpectationSource {
                                    expr_idx: 207,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        85,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 69,
                                src: ExpectationSource {
                                    expr_idx: 210,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        87,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 70,
                                src: ExpectationSource {
                                    expr_idx: 211,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 71,
                                src: ExpectationSource {
                                    expr_idx: 222,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        88,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 72,
                                src: ExpectationSource {
                                    expr_idx: 224,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        88,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 73,
                                src: ExpectationSource {
                                    expr_idx: 225,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 74,
                                src: ExpectationSource {
                                    expr_idx: 227,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        89,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 75,
                                src: ExpectationSource {
                                    expr_idx: 228,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 76,
                                src: ExpectationSource {
                                    expr_idx: 229,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 97,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 77,
                                src: ExpectationSource {
                                    expr_idx: 230,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 97,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
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
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    4,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            EtherealTerm(`TypeOntology(core::option::Option) TypeOntology(core::num::f32)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        PrefixOperandTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    4,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            EtherealTerm(`TypeOntology(core::option::Option) TypeOntology(core::num::f32)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    0,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    1,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        PrefixOperandTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
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
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    4,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    8,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 37,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            EtherealTerm(`TypeOntology(core::option::Option) TypeOntology(core::num::f32)`),
        ),
        self_ty: None,
    },
]