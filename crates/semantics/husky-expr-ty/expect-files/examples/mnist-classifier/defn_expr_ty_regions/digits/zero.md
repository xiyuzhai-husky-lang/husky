[
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                    3,
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
                                    expr_idx: 3,
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
                            path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
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
                            Hollow(
                                HollowTerm(
                                    1,
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
                        Ok(
                            Hollow(
                                HollowTerm(
                                    2,
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
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    None,
                ],
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
                                    2,
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
                                    3,
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
                                    10,
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
                                    expr_idx: 2,
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
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 3,
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
                            expectation: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 2,
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
                                    expr_idx: 6,
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
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        2,
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
                            path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 23,
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 24,
                                    },
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
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 79,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
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
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Method(
                            FluffyDotDisambiguation {
                                indirections: [],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        params: [],
                                        return_ty: EntityPath(
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
                            },
                        ),
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
                                            value: 51,
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
                    6,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 25,
                                    },
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
                disambiguation_and_ty_result: Err(
                    Derived(
                        FluffyTermError(
                            Todo,
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
                                    0,
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
                    9,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 25,
                                    },
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
                        FluffyTermError(
                            Todo,
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
                                    1,
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
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 30,
                                    },
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
                disambiguation_and_ty_result: Ok(
                    (
                        Method(
                            FluffyDotDisambiguation {
                                indirections: [
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        params: [],
                                        return_ty: EntityPath(
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
                            },
                        ),
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
                                            value: 51,
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
                    16,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 25,
                                    },
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
                        FluffyTermError(
                            Todo,
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
                                    2,
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
                        MethodOwnerTypeNotInferred,
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
                        Ok(
                            Hollow(
                                HollowTerm(
                                    3,
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
                    20,
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
                                            value: 96,
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
                                    5,
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
                    23,
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
                    24,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    11,
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
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Err(
                            Original(
                                CannotUnveil,
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
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    13,
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
                    27,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 24,
                                    },
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
                disambiguation_and_ty_result: Ok(
                    (
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 79,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 78,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 78,
                                        },
                                    ),
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
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [],
                                ty_path: TypePath(
                                    Id {
                                        value: 78,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 19,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 19,
                                    },
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
                            Hollow(
                                HollowTerm(
                                    14,
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
                        IndexOrComposeWithList(
                            Index(
                                FluffyDotDisambiguation {
                                    indirections: [],
                                    signature: FluffyIndexSignature {
                                        element_ty: Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 18,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 18,
                                    },
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
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 24,
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
                disambiguation_and_ty_result: Ok(
                    (
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 79,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 78,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 78,
                                        },
                                    ),
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
                disambiguation_and_ty_result: Ok(
                    (
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [],
                                ty_path: TypePath(
                                    Id {
                                        value: 78,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 19,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 19,
                                    },
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
                                    15,
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
                        IndexOrComposeWithList(
                            Index(
                                FluffyDotDisambiguation {
                                    indirections: [],
                                    signature: FluffyIndexSignature {
                                        element_ty: Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 18,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 18,
                                    },
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
                    41,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 24,
                                    },
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
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 79,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 78,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 78,
                                        },
                                    ),
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
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [],
                                ty_path: TypePath(
                                    Id {
                                        value: 78,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 19,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 19,
                                    },
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
                        Ok(
                            Hollow(
                                HollowTerm(
                                    16,
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
                        IndexOrComposeWithList(
                            Index(
                                FluffyDotDisambiguation {
                                    indirections: [],
                                    signature: FluffyIndexSignature {
                                        element_ty: Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 18,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 18,
                                    },
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 28,
                                    },
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
                disambiguation_and_ty_result: Err(
                    Derived(
                        FluffyTermError(
                            Todo,
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
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 28,
                                    },
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
                disambiguation_and_ty_result: Err(
                    Derived(
                        FluffyTermError(
                            Todo,
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
                                    17,
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
                    51,
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 96,
                                        },
                                    ),
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
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 96,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    53,
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
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
                    None,
                    None,
                    None,
                    None,
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    1,
                                ),
                            ),
                        ),
                    ),
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    2,
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
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            TypeOntologyAtPlace {
                                path: TypePath(
                                    Id {
                                        value: 48,
                                    },
                                ),
                                refined_path: Left(
                                    Borrow(
                                        Leash,
                                    ),
                                ),
                                arguments: [
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 22,
                                            },
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                ),
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                            },
                            TypeOntologyAtPlace {
                                path: TypePath(
                                    Id {
                                        value: 64,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 64,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 12,
                                            },
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            5,
                                        ),
                                    ),
                                },
                            },
                            TypeOntologyAtPlace {
                                path: TypePath(
                                    Id {
                                        value: 64,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 64,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 12,
                                            },
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            6,
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    10,
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
                                    14,
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
                                    23,
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
                                    28,
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
                                    33,
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
                                            4,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    23,
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
                                        value: 95,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 95,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            6,
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
                                        7,
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
                                        6,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        8,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    23,
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
                                        value: 95,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 95,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            6,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            10,
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
                            data: Hole {
                                hole_source: Expr(
                                    47,
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
                                    52,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [
                                    ImplicitlyConvertibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                14,
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
                                    58,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [
                                    ImplicitlyConvertibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                15,
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
                                    64,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [
                                    ImplicitlyConvertibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                16,
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
                                    84,
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
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 23,
                                        },
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
                                    expr_idx: 1,
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
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 24,
                                        },
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
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 15,
                                        },
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
                                idx: 4,
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
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
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
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
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
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 25,
                                        },
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
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 10,
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
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 11,
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
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 25,
                                        },
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
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 14,
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
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 16,
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
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 17,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 30,
                                        },
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
                                idx: 14,
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
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 19,
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
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 20,
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
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 21,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 25,
                                        },
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
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 23,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        2,
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
                                    expr_idx: 28,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        3,
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
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 29,
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
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 30,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 96,
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
                                idx: 22,
                                src: ExpectationSource {
                                    expr_idx: 33,
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
                            expectation: EqsRitchieType(
                                ExpectEqsRitchieType {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 23,
                                src: ExpectationSource {
                                    expr_idx: 35,
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
                                                                6,
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
                                                                10,
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
                                                        11,
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
                                idx: 24,
                                src: ExpectationSource {
                                    expr_idx: 42,
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
                                idx: 25,
                                src: ExpectationSource {
                                    expr_idx: 43,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        11,
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
                                idx: 26,
                                src: ExpectationSource {
                                    expr_idx: 47,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        13,
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
                                idx: 27,
                                src: ExpectationSource {
                                    expr_idx: 48,
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
                                idx: 28,
                                src: ExpectationSource {
                                    expr_idx: 49,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 24,
                                        },
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
                                    expr_idx: 50,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 78,
                                            },
                                        ),
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
                                idx: 30,
                                src: ExpectationSource {
                                    expr_idx: 51,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 19,
                                        },
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
                                idx: 31,
                                src: ExpectationSource {
                                    expr_idx: 52,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        14,
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
                                                    value: 60,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 32,
                                src: ExpectationSource {
                                    expr_idx: 53,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        14,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 33,
                                src: ExpectationSource {
                                    expr_idx: 53,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 18,
                                        },
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
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 35,
                                src: ExpectationSource {
                                    expr_idx: 55,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 24,
                                        },
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
                                idx: 36,
                                src: ExpectationSource {
                                    expr_idx: 56,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 78,
                                            },
                                        ),
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
                                idx: 37,
                                src: ExpectationSource {
                                    expr_idx: 57,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 19,
                                        },
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
                                idx: 38,
                                src: ExpectationSource {
                                    expr_idx: 58,
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
                            expectation: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    contract: None,
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 60,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 39,
                                src: ExpectationSource {
                                    expr_idx: 59,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        15,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 40,
                                src: ExpectationSource {
                                    expr_idx: 59,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 18,
                                        },
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
                                idx: 41,
                                src: ExpectationSource {
                                    expr_idx: 60,
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
                                idx: 42,
                                src: ExpectationSource {
                                    expr_idx: 61,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 24,
                                        },
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
                                idx: 43,
                                src: ExpectationSource {
                                    expr_idx: 62,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 78,
                                            },
                                        ),
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
                                idx: 44,
                                src: ExpectationSource {
                                    expr_idx: 63,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 19,
                                        },
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
                                idx: 45,
                                src: ExpectationSource {
                                    expr_idx: 64,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        16,
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
                                                    value: 60,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 46,
                                src: ExpectationSource {
                                    expr_idx: 65,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        16,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 47,
                                src: ExpectationSource {
                                    expr_idx: 65,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 18,
                                        },
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
                                idx: 48,
                                src: ExpectationSource {
                                    expr_idx: 73,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 28,
                                        },
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
                                idx: 49,
                                src: ExpectationSource {
                                    expr_idx: 75,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 28,
                                        },
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
                                idx: 50,
                                src: ExpectationSource {
                                    expr_idx: 84,
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
                                idx: 51,
                                src: ExpectationSource {
                                    expr_idx: 85,
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
                                idx: 52,
                                src: ExpectationSource {
                                    expr_idx: 88,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 96,
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
                                idx: 53,
                                src: ExpectationSource {
                                    expr_idx: 89,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 96,
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
]