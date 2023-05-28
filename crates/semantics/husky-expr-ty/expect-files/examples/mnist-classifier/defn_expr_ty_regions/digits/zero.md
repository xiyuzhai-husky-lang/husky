[
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Err(
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
                variant_and_ty_result: Err(
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
                variant_and_ty_result: Err(
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
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    3,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 59,
                                    },
                                ),
                                refined_path: Right(
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
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 3,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    1,
                                ),
                            ),
                            data: AnyDerived(
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
                        FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
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
                expectation_rule_idx: Some(
                    3,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        PrefixOperandTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
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
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    2,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
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
                            },
                            resolve_progress: Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [],
                    },
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 2,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    0,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 3,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    1,
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 5,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    1,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 6,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                            ),
                            data: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    parameter_contracted_ty: FluffyTermRitchieParameterContractedType {
                                        contract: Pure,
                                        ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        Trivial,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 10,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    2,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
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
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
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
                    0,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
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
                expectation_rule_idx: Some(
                    1,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 17,
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
                variant_and_ty_result: Err(
                    Original(
                        NoSuchField {
                            owner_ty: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 17,
                                    },
                                ),
                            ),
                            ident_token: IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 211,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    61,
                                ),
                            },
                        },
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                    3,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
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
                expectation_rule_idx: Some(
                    4,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
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
                    5,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        FluffyTermError(
                            Todo,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
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
                    6,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
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
                expectation_rule_idx: Some(
                    7,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
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
                    8,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        FluffyTermError(
                            Todo,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                    9,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
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
                expectation_rule_idx: Some(
                    10,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
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
                    11,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        Method(
                            FluffyMemberDisambiguation {
                                indirections: [
                                    Leash,
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 59,
                                    },
                                ),
                                signature: MethodFn(
                                    FluffyMethodFnSignature {
                                        nonself_parameter_contracted_tys: [],
                                        return_ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 46,
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
                                            value: 46,
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
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 46,
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
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
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
                expectation_rule_idx: Some(
                    14,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
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
                    15,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        FluffyTermError(
                            Todo,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                    16,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    4,
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
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
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
                expectation_rule_idx: Some(
                    18,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 92,
                                        },
                                    ),
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
                variant_and_ty_result: Err(
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
                variant_and_ty_result: Err(
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
                variant_and_ty_result: Ok(
                    (
                        List(
                            NewList,
                        ),
                        Ok(
                            Hollow(
                                HollowTerm(
                                    6,
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
                variant_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        ExprError,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    7,
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
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 35,
                                        },
                                    ),
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
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        Trivial,
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
                    23,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
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
                expectation_rule_idx: Some(
                    24,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 17,
                                    },
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
                variant_and_ty_result: Err(
                    Original(
                        NoSuchField {
                            owner_ty: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 17,
                                    },
                                ),
                            ),
                            ident_token: IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 213,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    159,
                                ),
                            },
                        },
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                    26,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
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
                expectation_rule_idx: Some(
                    27,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 17,
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
                variant_and_ty_result: Err(
                    Original(
                        NoSuchField {
                            owner_ty: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 17,
                                    },
                                ),
                            ),
                            ident_token: IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 213,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    170,
                                ),
                            },
                        },
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    10,
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
                variant_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
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
                expectation_rule_idx: Some(
                    30,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 17,
                                    },
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
                variant_and_ty_result: Err(
                    Original(
                        NoSuchField {
                            owner_ty: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 17,
                                    },
                                ),
                            ),
                            ident_token: IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 213,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    183,
                                ),
                            },
                        },
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                    32,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        ApplicationOrRitchieCallFunctionTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 21,
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
                variant_and_ty_result: Err(
                    Derived(
                        FluffyTermError(
                            Todo,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 21,
                                    },
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
                variant_and_ty_result: Err(
                    Derived(
                        FluffyTermError(
                            Todo,
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        MethodOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    12,
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
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
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
                expectation_rule_idx: Some(
                    36,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                variant_and_ty_result: Err(
                    Derived(
                        FieldOwnerTypeNotInferred,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 92,
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
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 92,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    38,
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
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    5,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
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
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 59,
                                    },
                                ),
                                refined_path: Right(
                                    List,
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            5,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    40,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    44,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    49,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    55,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    61,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
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
                            },
                            resolve_progress: Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            0,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 43,
                                    },
                                ),
                                refined_path: Right(
                                    Borrow(
                                        Leash,
                                    ),
                                ),
                                arguments: [
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 14,
                                            },
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 0,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 15,
                                    },
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 1,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                            ),
                            data: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    parameter_contracted_ty: FluffyTermRitchieParameterContractedType {
                                        contract: Pure,
                                        ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        Trivial,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 2,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 17,
                                    },
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 5,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    0,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 6,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                            ),
                            data: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    parameter_contracted_ty: FluffyTermRitchieParameterContractedType {
                                        contract: Pure,
                                        ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        Trivial,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 7,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 18,
                                    },
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 10,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    1,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 11,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                            ),
                            data: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    parameter_contracted_ty: FluffyTermRitchieParameterContractedType {
                                        contract: Pure,
                                        ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        Trivial,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 12,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 18,
                                    },
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 14,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    2,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 16,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                            ),
                            data: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    parameter_contracted_ty: FluffyTermRitchieParameterContractedType {
                                        contract: Pure,
                                        ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        Trivial,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 17,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 18,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 46,
                                        },
                                    ),
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 19,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 46,
                                        },
                                    ),
                                ),
                            ),
                            data: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    parameter_contracted_ty: FluffyTermRitchieParameterContractedType {
                                        contract: Pure,
                                        ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 46,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        Trivial,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 20,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                            ),
                            data: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    parameter_contracted_ty: FluffyTermRitchieParameterContractedType {
                                        contract: Pure,
                                        ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        Trivial,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 21,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 18,
                                    },
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 23,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    3,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 28,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    4,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 29,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                            ),
                            data: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    parameter_contracted_ty: FluffyTermRitchieParameterContractedType {
                                        contract: Pure,
                                        ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        Trivial,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 30,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 92,
                                        },
                                    ),
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 33,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    6,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 40,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    7,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 41,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 35,
                                        },
                                    ),
                                ),
                            ),
                            data: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    parameter_contracted_ty: FluffyTermRitchieParameterContractedType {
                                        contract: Pure,
                                        ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 35,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        Trivial,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 44,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    8,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 45,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                            ),
                            data: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    parameter_contracted_ty: FluffyTermRitchieParameterContractedType {
                                        contract: Pure,
                                        ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        Trivial,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 46,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 17,
                                    },
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 49,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    9,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 51,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                            ),
                            data: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    parameter_contracted_ty: FluffyTermRitchieParameterContractedType {
                                        contract: Pure,
                                        ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        Trivial,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 52,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 17,
                                    },
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 55,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    10,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 57,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                            ),
                            data: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    parameter_contracted_ty: FluffyTermRitchieParameterContractedType {
                                        contract: Pure,
                                        ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        Trivial,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 58,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 17,
                                    },
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 61,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    11,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 70,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 72,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 81,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    12,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 82,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                            ),
                            data: ImplicitlyConvertible(
                                ExpectImplicitlyConvertible {
                                    parameter_contracted_ty: FluffyTermRitchieParameterContractedType {
                                        contract: Pure,
                                        ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    ImplicitlyConvertible(
                                        Trivial,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 85,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 92,
                                        },
                                    ),
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 86,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 92,
                                        },
                                    ),
                                ),
                            ),
                            data: AnyDerived(
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
]