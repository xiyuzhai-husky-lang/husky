[
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
                    ),
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
                        FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                    ),
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
    ExprTypeRegion {
        path: RegionPath::Defn(
            DefnRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
                    ),
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
                                    2,
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
                                            value: 33,
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
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    16,
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
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 13,
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
                                expr_idx: 17,
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