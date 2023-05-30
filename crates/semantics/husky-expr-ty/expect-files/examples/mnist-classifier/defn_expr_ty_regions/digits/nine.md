[
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
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
                        FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
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
                        FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
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
                                        value: 15,
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
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 16,
                                    },
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
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 76,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 75,
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
                                            value: 75,
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
                                    0,
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
                                        value: 17,
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
                                    1,
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
                        MethodOwnerTypeNotInferred,
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
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 16,
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
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 16,
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
                variant_and_ty_result: Ok(
                    (
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 76,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 56,
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
                                            value: 56,
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
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 76,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 56,
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
                                            value: 56,
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
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 56,
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
                                    2,
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
                    17,
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
                                    3,
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
                                    4,
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
                    21,
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
                    22,
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
                                    5,
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
                                    6,
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
                                        value: 16,
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
                variant_and_ty_result: Ok(
                    (
                        TypePath(
                            Constructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 16,
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
                variant_and_ty_result: Ok(
                    (
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 76,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 56,
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
                                            value: 56,
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
                variant_and_ty_result: Ok(
                    (
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 76,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 56,
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
                                            value: 56,
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
                variant_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 56,
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
                    34,
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
                    35,
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
                        MethodOwnerTypeNotInferred,
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
                    36,
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
                    37,
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
                    38,
                ),
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
                    39,
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
                    40,
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
                    41,
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
                                    10,
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
                    43,
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
                                        value: 16,
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
                    45,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        Method(
                            FluffyMemberDisambiguation {
                                indirections: [
                                    Leash,
                                ],
                                signature: MethodFn(
                                    FluffyMethodFnSignature {
                                        nonself_parameter_contracted_tys: [
                                            FluffyTermRitchieParameterContractedType {
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
                                        ],
                                        return_ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 56,
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
                                            value: 56,
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
                                    11,
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
                    48,
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
                    49,
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
                    50,
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
                    51,
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
                    52,
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
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    1,
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
                                    8,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    13,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    26,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    30,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    35,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
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
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    45,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    54,
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
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    64,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    70,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    76,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    79,
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
                                refined_path: Left(
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
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            1,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 43,
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
                                expr_idx: 3,
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
                                expr_idx: 4,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 16,
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
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 75,
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
                                expr_idx: 8,
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
                                expr_idx: 10,
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
                                expr_idx: 11,
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
                                expr_idx: 13,
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
                                expr_idx: 20,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 16,
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
                                expr_idx: 22,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 56,
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
                                expr_idx: 21,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 16,
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
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 56,
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
                                                        value: 56,
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
                                expr_idx: 24,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 56,
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
                                expr_idx: 26,
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
                                expr_idx: 27,
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
                                expr_idx: 32,
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
                                expr_idx: 35,
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
                                expr_idx: 36,
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
                                expr_idx: 37,
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
                                expr_idx: 39,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    5,
                                ),
                            ),
                            data: AnyDerived(
                                ExpectAnyDerived,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 42,
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
                                expr_idx: 43,
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
                                expr_idx: 45,
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
                                expr_idx: 46,
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
                                expr_idx: 47,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 16,
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
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 56,
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
                                expr_idx: 48,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 16,
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
                                expr_idx: 50,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 56,
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
                                                        value: 56,
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
                                expr_idx: 51,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 56,
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
                                expr_idx: 54,
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
                                expr_idx: 61,
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
                                expr_idx: 62,
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
                                expr_idx: 64,
                                kind: Expr,
                            },
                            expectee: Hollow(
                                HollowTerm(
                                    9,
                                ),
                            ),
                            data: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 66,
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
                                expr_idx: 67,
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
                                expr_idx: 68,
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
                                expr_idx: 70,
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
                                expr_idx: 71,
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
                                expr_idx: 72,
                                kind: Expr,
                            },
                            expectee: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 16,
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
                                expr_idx: 73,
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
                                expr_idx: 74,
                                kind: Expr,
                            },
                            expectee: EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 56,
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
                                expr_idx: 76,
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
                                expr_idx: 77,
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
                                expr_idx: 79,
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
                                expr_idx: 80,
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
                                expr_idx: 81,
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
                                expr_idx: 82,
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
                                expr_idx: 83,
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
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FugitivePath(`mnist_classifier::digits::nine::downmost`, `Fn`),
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
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    4,
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
                                expr_idx: 4,
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
                                expr_idx: 5,
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
                        FugitivePath(`mnist_classifier::digits::nine::big_cc`, `Fn`),
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
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    4,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    9,
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
                                expr_idx: 4,
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
                                expr_idx: 5,
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
                                expr_idx: 9,
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
                                expr_idx: 10,
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