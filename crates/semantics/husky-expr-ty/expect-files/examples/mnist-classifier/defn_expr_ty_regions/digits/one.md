[
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntitySynNodePath::ModuleItem(
                ModuleItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
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
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Ritchie(
                                EtherealTermRitchie(
                                    Id {
                                        value: 11,
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 46,
                                    },
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
                            Ritchie(
                                EtherealTermRitchie(
                                    Id {
                                        value: 10,
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Ritchie(
                                EtherealTermRitchie(
                                    Id {
                                        value: 10,
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Ritchie(
                                EtherealTermRitchie(
                                    Id {
                                        value: 10,
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
                disambiguation_and_ty_result: Ok(
                    (
                        List(
                            NewList,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 48,
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
                disambiguation_and_ty_result: Ok(
                    (
                        ExplicitApplicationOrFunctionCall(
                            RitchieCall,
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
                    6,
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
                    7,
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
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: Ritchie(
                                    EtherealTermRitchie(
                                        Id {
                                            value: 11,
                                        },
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                implicit_parameter_substitutions: [],
                                                return_ty: EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 96,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                variant: Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: Application(
                                                                    EtherealTermApplication(
                                                                        Id {
                                                                            value: 46,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: Application(
                                                                    EtherealTermApplication(
                                                                        Id {
                                                                            value: 48,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 46,
                                            },
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
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 46,
                                        },
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Ritchie(
                                        EtherealTermRitchie(
                                            Id {
                                                value: 10,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: Ritchie(
                                    EtherealTermRitchie(
                                        Id {
                                            value: 10,
                                        },
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Ritchie(
                                        EtherealTermRitchie(
                                            Id {
                                                value: 10,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: Ritchie(
                                    EtherealTermRitchie(
                                        Id {
                                            value: 10,
                                        },
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Ritchie(
                                        EtherealTermRitchie(
                                            Id {
                                                value: 10,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: Ritchie(
                                    EtherealTermRitchie(
                                        Id {
                                            value: 10,
                                        },
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 48,
                                            },
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
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 48,
                                        },
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 96,
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
                                                value: 96,
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 96,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
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
            EtherealTerm(`FermiMatchResult`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntitySynNodePath::ModuleItem(
                ModuleItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
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
                                        value: 18,
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 71,
                                    },
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
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 81,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
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
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 44,
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
                            Hollow(
                                HollowTerm(
                                    5,
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
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 39,
                                        },
                                    ),
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Ritchie(
                                EtherealTermRitchie(
                                    Id {
                                        value: 11,
                                    },
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
                                        value: 46,
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
                disambiguation_and_ty_result: Ok(
                    (
                        List(
                            NewList,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 48,
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
                disambiguation_and_ty_result: Ok(
                    (
                        ExplicitApplicationOrFunctionCall(
                            RitchieCall,
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
                    12,
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
                                    7,
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
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 41,
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
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
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 71,
                                    },
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
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 81,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
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
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 44,
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
                    19,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 39,
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
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 71,
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
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 81,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
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
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
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
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 41,
                                        },
                                    ),
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 71,
                                    },
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
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 81,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
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
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
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
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 41,
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
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 63,
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
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 71,
                                    },
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
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 81,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
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
                                            value: 60,
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
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 41,
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
                                            value: 60,
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
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
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
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 41,
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
                                        value: 84,
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
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 86,
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
                                        value: 86,
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
                            Hollow(
                                HollowTerm(
                                    15,
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
                        IndexOrComposeWithList(
                            Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: FluffyIndexSignature {
                                        element_ty: Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 85,
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
                                        value: 85,
                                    },
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 84,
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
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 86,
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
                                        value: 86,
                                    },
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
                            Hollow(
                                HollowTerm(
                                    16,
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
                        IndexOrComposeWithList(
                            Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: FluffyIndexSignature {
                                        element_ty: Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 85,
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
                                        value: 85,
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
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 84,
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
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 86,
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
                                        value: 86,
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
                    51,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        IndexOrComposeWithList(
                            Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: FluffyIndexSignature {
                                        element_ty: Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 85,
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
                                        value: 85,
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
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 41,
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
                                    18,
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
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 41,
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
                                        value: 18,
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
                                            value: 44,
                                        },
                                    ),
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
                    59,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 39,
                                        },
                                    ),
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 71,
                                    },
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
                        Ok(
                            Hollow(
                                HollowTerm(
                                    26,
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
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 81,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 71,
                                    },
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
                                    26,
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
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 81,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
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
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    26,
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
                            Hollow(
                                HollowTerm(
                                    26,
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
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 41,
                                        },
                                    ),
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 63,
                                    },
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 84,
                                    },
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
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
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
                                            value: 60,
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
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 41,
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 18,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    77,
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
                                        value: 71,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    78,
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
                                            value: 44,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    79,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenic_parameters: [
                                            Regular(
                                                FluffyTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 44,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ],
                                        return_ty: EntityPath(
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    80,
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
                                        value: 84,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    81,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    82,
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
                                        value: 84,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    83,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    84,
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
                                        value: 84,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    85,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    86,
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
                                            value: 44,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    87,
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
                    88,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    91,
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
                                        value: 18,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    92,
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
                                        value: 84,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    93,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 86,
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
                                        value: 86,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    94,
                ),
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
                    95,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        IndexOrComposeWithList(
                            Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: FluffyIndexSignature {
                                        element_ty: Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 85,
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
                                        value: 85,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    97,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnwrapOrComposeWithNot(
                            Unwrap,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    98,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 93,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    99,
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
                                        value: 84,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    100,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 86,
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
                                        value: 86,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    101,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    42,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    102,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        IndexOrComposeWithList(
                            Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: FluffyIndexSignature {
                                        element_ty: Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 85,
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
                                        value: 85,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    104,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnwrapOrComposeWithNot(
                            Unwrap,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    105,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 93,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    106,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenic_parameters: [],
                                        return_ty: EntityPath(
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    107,
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
                                            value: 44,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    108,
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
                    109,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    112,
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
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 41,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    113,
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
                                    43,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    114,
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
                                            value: 41,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    115,
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
                                    44,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    116,
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
                                    45,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    117,
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
                                            value: 41,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    118,
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
                                        value: 18,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    119,
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
                                            value: 44,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    120,
                ),
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
                    121,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    124,
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
                                        value: 18,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    125,
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
                                            value: 44,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    126,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    58,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    127,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    130,
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
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 41,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    131,
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
                                        value: 18,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    132,
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
                                        value: 84,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    133,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 86,
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
                                        value: 86,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    134,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    67,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    135,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        IndexOrComposeWithList(
                            Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: FluffyIndexSignature {
                                        element_ty: Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 85,
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
                                        value: 85,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    137,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnwrapOrComposeWithNot(
                            Unwrap,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    138,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 93,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    139,
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
                                        value: 84,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    140,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 86,
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
                                        value: 86,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    141,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    68,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    142,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        IndexOrComposeWithList(
                            Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: FluffyIndexSignature {
                                        element_ty: Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 85,
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
                                        value: 85,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    144,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnwrapOrComposeWithNot(
                            Unwrap,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    145,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 93,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    146,
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
                                        value: 84,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    147,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 96,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 86,
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
                                        value: 86,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    148,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    69,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    149,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        IndexOrComposeWithList(
                            Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: FluffyIndexSignature {
                                        element_ty: Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 85,
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
                                        value: 85,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    151,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnwrapOrComposeWithNot(
                            Unwrap,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    152,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 93,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    153,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenic_parameters: [],
                                        return_ty: EntityPath(
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    154,
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
                                            value: 44,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    155,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Hollow(
                                HollowTerm(
                                    65,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    156,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    159,
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
                                    70,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    160,
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
                                            value: 41,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    161,
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
                                    71,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    162,
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
                                        value: 18,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    163,
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
                                            value: 44,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    164,
                ),
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
                    165,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        UnveilOrComposeWithOption(
                            Unveil,
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    168,
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
                                    79,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    169,
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
                                            value: 41,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    170,
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
                                        value: 71,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    171,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 81,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    172,
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
                                        value: 71,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    173,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 81,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    174,
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
                                    80,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    175,
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
                                    80,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    176,
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
                                            value: 41,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    177,
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
                                    81,
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    178,
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
                                            value: 41,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    179,
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
                                        value: 63,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    180,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 63,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    181,
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
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                fill: Some(
                                    EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                constraints: [],
                            },
                            resolve_progress: ResolvedEthereal(
                                EntityPath(
                                    TypeVariant(
                                        TypeVariantPath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                    15,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                8,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            8,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        9,
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
                                        8,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        10,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    15,
                                ),
                                hole_kind: Any,
                                fill: Some(
                                    EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                constraints: [],
                            },
                            resolve_progress: ResolvedEthereal(
                                EntityPath(
                                    TypeVariant(
                                        TypeVariantPath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            8,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            12,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        13,
                                    ),
                                ),
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
                                constraints: [
                                    CoercibleTo {
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
                                    43,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
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
                                    47,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                17,
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
                                    57,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                19,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            19,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        20,
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
                                        19,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        21,
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
                                fill: Some(
                                    EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                constraints: [],
                            },
                            resolve_progress: ResolvedEthereal(
                                EntityPath(
                                    TypeVariant(
                                        TypeVariantPath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            19,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            23,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        24,
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
                                    CoercibleFrom {
                                        target: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 60,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    CoercibleFrom {
                                        target: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 60,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    77,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                27,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            27,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        28,
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
                                        27,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        29,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    77,
                                ),
                                hole_kind: Any,
                                fill: Some(
                                    EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                constraints: [],
                            },
                            resolve_progress: ResolvedEthereal(
                                EntityPath(
                                    TypeVariant(
                                        TypeVariantPath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            27,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            31,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        32,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    92,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                34,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                    92,
                                ),
                                hole_kind: Any,
                                fill: Some(
                                    EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                constraints: [],
                            },
                            resolve_progress: ResolvedEthereal(
                                EntityPath(
                                    TypeVariant(
                                        TypeVariantPath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                    99,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                41,
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
                                    105,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                42,
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
                                    122,
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
                                    129,
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
                                    136,
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
                                    119,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                46,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            46,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        46,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        48,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    119,
                                ),
                                hole_kind: Any,
                                fill: Some(
                                    EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                constraints: [],
                            },
                            resolve_progress: ResolvedEthereal(
                                EntityPath(
                                    TypeVariant(
                                        TypeVariantPath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            46,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            50,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        51,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    125,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                53,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            53,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        54,
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
                                        53,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        55,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    125,
                                ),
                                hole_kind: Any,
                                fill: Some(
                                    EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                constraints: [],
                            },
                            resolve_progress: ResolvedEthereal(
                                EntityPath(
                                    TypeVariant(
                                        TypeVariantPath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            53,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            57,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        58,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    132,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                60,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            60,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        61,
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
                                        60,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        62,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    132,
                                ),
                                hole_kind: Any,
                                fill: Some(
                                    EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                constraints: [],
                            },
                            resolve_progress: ResolvedEthereal(
                                EntityPath(
                                    TypeVariant(
                                        TypeVariantPath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            60,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            64,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        65,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    165,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                67,
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
                                    171,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                68,
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
                                    177,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                69,
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
                                    186,
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
                                    201,
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
                                    163,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                72,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            72,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        72,
                                    ),
                                ),
                                return_ty: Hollow(
                                    HollowTerm(
                                        74,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    163,
                                ),
                                hole_kind: Any,
                                fill: Some(
                                    EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                constraints: [],
                            },
                            resolve_progress: ResolvedEthereal(
                                EntityPath(
                                    TypeVariant(
                                        TypeVariantPath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 98,
                                    },
                                ),
                                refined_path: Right(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 98,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            72,
                                        ),
                                    ),
                                    Hollow(
                                        HollowTerm(
                                            76,
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
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Keyed(
                                        FluffyTermRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 448,
                                                    },
                                                ),
                                            ),
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 44,
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
                                        77,
                                    ),
                                ),
                            },
                            resolve_progress: Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    215,
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
                                    227,
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
                                    232,
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
                                            value: 18,
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
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Coword(
                                                                    Id {
                                                                        value: 448,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 44,
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
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 71,
                                        },
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
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
                                                value: 44,
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 91,
                                            },
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
                                expectee: Hollow(
                                    HollowTerm(
                                        5,
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
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 99,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expectation(
                                        5,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expectation(
                                        5,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        4,
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        Subtype(
                                            ExpectSubtypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 39,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 39,
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
                            expectation: EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: Ritchie(
                                    EtherealTermRitchie(
                                        Id {
                                            value: 11,
                                        },
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                implicit_parameter_substitutions: [],
                                                return_ty: EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 96,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                variant: Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: Application(
                                                                    EtherealTermApplication(
                                                                        Id {
                                                                            value: 46,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: Application(
                                                                    EtherealTermApplication(
                                                                        Id {
                                                                            value: 48,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 46,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 46,
                                        },
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 48,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 48,
                                        },
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
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 10,
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
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 13,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        7,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 14,
                                src: ExpectationSource {
                                    expr_idx: 14,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 15,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 18,
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
                                                                8,
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
                                                                12,
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
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Coword(
                                                                    Id {
                                                                        value: 448,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 44,
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
                                                        13,
                                                    ),
                                                ),
                                            },
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
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 16,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 71,
                                        },
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 17,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 18,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 44,
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 91,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 19,
                                src: ExpectationSource {
                                    expr_idx: 19,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        13,
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
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 99,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 19,
                                    kind: Expectation(
                                        19,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        8,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 19,
                                    kind: Expectation(
                                        19,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        12,
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        Subtype(
                                            ExpectSubtypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 39,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 22,
                                src: ExpectationSource {
                                    expr_idx: 20,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 39,
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
                                idx: 23,
                                src: ExpectationSource {
                                    expr_idx: 21,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 71,
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
                                idx: 24,
                                src: ExpectationSource {
                                    expr_idx: 22,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 25,
                                src: ExpectationSource {
                                    expr_idx: 23,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 26,
                                src: ExpectationSource {
                                    expr_idx: 24,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 27,
                                src: ExpectationSource {
                                    expr_idx: 25,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 71,
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
                                idx: 28,
                                src: ExpectationSource {
                                    expr_idx: 26,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 29,
                                src: ExpectationSource {
                                    expr_idx: 27,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 30,
                                src: ExpectationSource {
                                    expr_idx: 28,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 63,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 31,
                                src: ExpectationSource {
                                    expr_idx: 29,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 63,
                                        },
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
                                idx: 32,
                                src: ExpectationSource {
                                    expr_idx: 30,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 71,
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
                                idx: 33,
                                src: ExpectationSource {
                                    expr_idx: 31,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 34,
                                src: ExpectationSource {
                                    expr_idx: 32,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 35,
                                src: ExpectationSource {
                                    expr_idx: 33,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 36,
                                src: ExpectationSource {
                                    expr_idx: 34,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 37,
                                src: ExpectationSource {
                                    expr_idx: 35,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 38,
                                src: ExpectationSource {
                                    expr_idx: 36,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 39,
                                src: ExpectationSource {
                                    expr_idx: 37,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 84,
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
                                idx: 40,
                                src: ExpectationSource {
                                    expr_idx: 38,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 86,
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
                                idx: 41,
                                src: ExpectationSource {
                                    expr_idx: 39,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 42,
                                src: ExpectationSource {
                                    expr_idx: 40,
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
                                idx: 43,
                                src: ExpectationSource {
                                    expr_idx: 40,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 85,
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
                                idx: 44,
                                src: ExpectationSource {
                                    expr_idx: 41,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 84,
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
                                    expr_idx: 42,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 86,
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
                                idx: 46,
                                src: ExpectationSource {
                                    expr_idx: 43,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 47,
                                src: ExpectationSource {
                                    expr_idx: 44,
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
                                idx: 48,
                                src: ExpectationSource {
                                    expr_idx: 44,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 85,
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
                                    expr_idx: 45,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 84,
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
                                idx: 50,
                                src: ExpectationSource {
                                    expr_idx: 46,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 86,
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
                                idx: 51,
                                src: ExpectationSource {
                                    expr_idx: 47,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 52,
                                src: ExpectationSource {
                                    expr_idx: 48,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        17,
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
                                idx: 53,
                                src: ExpectationSource {
                                    expr_idx: 48,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 85,
                                        },
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 54,
                                src: ExpectationSource {
                                    expr_idx: 50,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 55,
                                src: ExpectationSource {
                                    expr_idx: 53,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 56,
                                src: ExpectationSource {
                                    expr_idx: 54,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                    expr_idx: 55,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 18,
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
                                                                19,
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
                                                                23,
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
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Coword(
                                                                    Id {
                                                                        value: 448,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 44,
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
                                                        24,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 58,
                                src: ExpectationSource {
                                    expr_idx: 59,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 44,
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 91,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 59,
                                src: ExpectationSource {
                                    expr_idx: 60,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        24,
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
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 99,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 60,
                                src: ExpectationSource {
                                    expr_idx: 60,
                                    kind: Expectation(
                                        59,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        19,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 61,
                                src: ExpectationSource {
                                    expr_idx: 60,
                                    kind: Expectation(
                                        59,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        23,
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        Subtype(
                                            ExpectSubtypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 39,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 62,
                                src: ExpectationSource {
                                    expr_idx: 61,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 39,
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
                                idx: 63,
                                src: ExpectationSource {
                                    expr_idx: 63,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        26,
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
                                idx: 64,
                                src: ExpectationSource {
                                    expr_idx: 62,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 71,
                                        },
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: Hollow(
                                        HollowTerm(
                                            26,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 65,
                                src: ExpectationSource {
                                    expr_idx: 64,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
                                            },
                                        ),
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
                                idx: 66,
                                src: ExpectationSource {
                                    expr_idx: 66,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        26,
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
                                idx: 67,
                                src: ExpectationSource {
                                    expr_idx: 65,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 71,
                                        },
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: Hollow(
                                        HollowTerm(
                                            26,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 68,
                                src: ExpectationSource {
                                    expr_idx: 67,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
                                            },
                                        ),
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
                                idx: 69,
                                src: ExpectationSource {
                                    expr_idx: 68,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        26,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: Hollow(
                                        HollowTerm(
                                            26,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 70,
                                src: ExpectationSource {
                                    expr_idx: 69,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        26,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 71,
                                src: ExpectationSource {
                                    expr_idx: 70,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 63,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 72,
                                src: ExpectationSource {
                                    expr_idx: 71,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 63,
                                        },
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
                                idx: 73,
                                src: ExpectationSource {
                                    expr_idx: 79,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 84,
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
                                idx: 74,
                                src: ExpectationSource {
                                    expr_idx: 80,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 75,
                                src: ExpectationSource {
                                    expr_idx: 81,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 76,
                                src: ExpectationSource {
                                    expr_idx: 82,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 77,
                                src: ExpectationSource {
                                    expr_idx: 83,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 18,
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
                                                                27,
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
                                                                31,
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
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Coword(
                                                                    Id {
                                                                        value: 448,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 44,
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
                                                        32,
                                                    ),
                                                ),
                                            },
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
                                idx: 78,
                                src: ExpectationSource {
                                    expr_idx: 84,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 71,
                                        },
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 79,
                                src: ExpectationSource {
                                    expr_idx: 85,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 44,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 80,
                                src: ExpectationSource {
                                    expr_idx: 86,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                idx: 81,
                                src: ExpectationSource {
                                    expr_idx: 87,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 84,
                                        },
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 82,
                                src: ExpectationSource {
                                    expr_idx: 88,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                idx: 83,
                                src: ExpectationSource {
                                    expr_idx: 89,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 84,
                                        },
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 84,
                                src: ExpectationSource {
                                    expr_idx: 90,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                idx: 85,
                                src: ExpectationSource {
                                    expr_idx: 91,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 84,
                                        },
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 86,
                                src: ExpectationSource {
                                    expr_idx: 92,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 87,
                                src: ExpectationSource {
                                    expr_idx: 93,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 44,
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 91,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 88,
                                src: ExpectationSource {
                                    expr_idx: 94,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        32,
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
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 99,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 89,
                                src: ExpectationSource {
                                    expr_idx: 94,
                                    kind: Expectation(
                                        88,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        27,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 90,
                                src: ExpectationSource {
                                    expr_idx: 94,
                                    kind: Expectation(
                                        88,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        31,
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        Subtype(
                                            ExpectSubtypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 39,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 91,
                                src: ExpectationSource {
                                    expr_idx: 95,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 39,
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
                                idx: 92,
                                src: ExpectationSource {
                                    expr_idx: 96,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 18,
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
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Coword(
                                                                    Id {
                                                                        value: 448,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 44,
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
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 93,
                                src: ExpectationSource {
                                    expr_idx: 97,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 84,
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
                                idx: 94,
                                src: ExpectationSource {
                                    expr_idx: 98,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 86,
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
                                idx: 95,
                                src: ExpectationSource {
                                    expr_idx: 99,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 96,
                                src: ExpectationSource {
                                    expr_idx: 100,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        41,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 97,
                                src: ExpectationSource {
                                    expr_idx: 100,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 85,
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
                                idx: 98,
                                src: ExpectationSource {
                                    expr_idx: 101,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 47,
                                        },
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 99,
                                src: ExpectationSource {
                                    expr_idx: 102,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                idx: 100,
                                src: ExpectationSource {
                                    expr_idx: 103,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 84,
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
                                idx: 101,
                                src: ExpectationSource {
                                    expr_idx: 104,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 86,
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
                                idx: 102,
                                src: ExpectationSource {
                                    expr_idx: 105,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        42,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 103,
                                src: ExpectationSource {
                                    expr_idx: 106,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        42,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 104,
                                src: ExpectationSource {
                                    expr_idx: 106,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 85,
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
                                idx: 105,
                                src: ExpectationSource {
                                    expr_idx: 107,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 47,
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
                                idx: 106,
                                src: ExpectationSource {
                                    expr_idx: 108,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 107,
                                src: ExpectationSource {
                                    expr_idx: 109,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 108,
                                src: ExpectationSource {
                                    expr_idx: 110,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 44,
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 91,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 109,
                                src: ExpectationSource {
                                    expr_idx: 111,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        39,
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
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 99,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 110,
                                src: ExpectationSource {
                                    expr_idx: 111,
                                    kind: Expectation(
                                        109,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        34,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 111,
                                src: ExpectationSource {
                                    expr_idx: 111,
                                    kind: Expectation(
                                        109,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        38,
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        Subtype(
                                            ExpectSubtypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 39,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 112,
                                src: ExpectationSource {
                                    expr_idx: 112,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 39,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 113,
                                src: ExpectationSource {
                                    expr_idx: 114,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 114,
                                src: ExpectationSource {
                                    expr_idx: 122,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        43,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 115,
                                src: ExpectationSource {
                                    expr_idx: 123,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 116,
                                src: ExpectationSource {
                                    expr_idx: 129,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        44,
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
                                idx: 117,
                                src: ExpectationSource {
                                    expr_idx: 136,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        45,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 118,
                                src: ExpectationSource {
                                    expr_idx: 137,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 119,
                                src: ExpectationSource {
                                    expr_idx: 138,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 18,
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
                                                                46,
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
                                                                50,
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
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Coword(
                                                                    Id {
                                                                        value: 448,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 44,
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
                                                        51,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 120,
                                src: ExpectationSource {
                                    expr_idx: 146,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 44,
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 91,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 121,
                                src: ExpectationSource {
                                    expr_idx: 147,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        51,
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
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 99,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 122,
                                src: ExpectationSource {
                                    expr_idx: 147,
                                    kind: Expectation(
                                        121,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        46,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 123,
                                src: ExpectationSource {
                                    expr_idx: 147,
                                    kind: Expectation(
                                        121,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        50,
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        Subtype(
                                            ExpectSubtypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 39,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 124,
                                src: ExpectationSource {
                                    expr_idx: 148,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 39,
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
                                idx: 125,
                                src: ExpectationSource {
                                    expr_idx: 149,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 18,
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
                                                                53,
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
                                                                57,
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
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Coword(
                                                                    Id {
                                                                        value: 448,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 44,
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
                                                        58,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 126,
                                src: ExpectationSource {
                                    expr_idx: 157,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 44,
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 91,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 127,
                                src: ExpectationSource {
                                    expr_idx: 158,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        58,
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
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 99,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 128,
                                src: ExpectationSource {
                                    expr_idx: 158,
                                    kind: Expectation(
                                        127,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        53,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 129,
                                src: ExpectationSource {
                                    expr_idx: 158,
                                    kind: Expectation(
                                        127,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        57,
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        Subtype(
                                            ExpectSubtypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 39,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 130,
                                src: ExpectationSource {
                                    expr_idx: 159,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 39,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 131,
                                src: ExpectationSource {
                                    expr_idx: 161,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 132,
                                src: ExpectationSource {
                                    expr_idx: 162,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 18,
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
                                                                60,
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
                                                                64,
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
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Coword(
                                                                    Id {
                                                                        value: 448,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 44,
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
                                                        65,
                                                    ),
                                                ),
                                            },
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
                                idx: 133,
                                src: ExpectationSource {
                                    expr_idx: 163,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 84,
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
                                idx: 134,
                                src: ExpectationSource {
                                    expr_idx: 164,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 86,
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
                                idx: 135,
                                src: ExpectationSource {
                                    expr_idx: 165,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        67,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 136,
                                src: ExpectationSource {
                                    expr_idx: 166,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        67,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 137,
                                src: ExpectationSource {
                                    expr_idx: 166,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 85,
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
                                idx: 138,
                                src: ExpectationSource {
                                    expr_idx: 167,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 47,
                                        },
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 139,
                                src: ExpectationSource {
                                    expr_idx: 168,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                idx: 140,
                                src: ExpectationSource {
                                    expr_idx: 169,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 84,
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
                                idx: 141,
                                src: ExpectationSource {
                                    expr_idx: 170,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 86,
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
                                idx: 142,
                                src: ExpectationSource {
                                    expr_idx: 171,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        68,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 143,
                                src: ExpectationSource {
                                    expr_idx: 172,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        68,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 144,
                                src: ExpectationSource {
                                    expr_idx: 172,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 85,
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
                                idx: 145,
                                src: ExpectationSource {
                                    expr_idx: 173,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 47,
                                        },
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 146,
                                src: ExpectationSource {
                                    expr_idx: 174,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                idx: 147,
                                src: ExpectationSource {
                                    expr_idx: 175,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 84,
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
                                idx: 148,
                                src: ExpectationSource {
                                    expr_idx: 176,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 86,
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
                                idx: 149,
                                src: ExpectationSource {
                                    expr_idx: 177,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        69,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 150,
                                src: ExpectationSource {
                                    expr_idx: 178,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        69,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 151,
                                src: ExpectationSource {
                                    expr_idx: 178,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 85,
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
                                idx: 152,
                                src: ExpectationSource {
                                    expr_idx: 179,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 47,
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
                                idx: 153,
                                src: ExpectationSource {
                                    expr_idx: 180,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
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
                                idx: 154,
                                src: ExpectationSource {
                                    expr_idx: 181,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 155,
                                src: ExpectationSource {
                                    expr_idx: 182,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 44,
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 91,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 156,
                                src: ExpectationSource {
                                    expr_idx: 183,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        65,
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
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 99,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 157,
                                src: ExpectationSource {
                                    expr_idx: 183,
                                    kind: Expectation(
                                        156,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        60,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 158,
                                src: ExpectationSource {
                                    expr_idx: 183,
                                    kind: Expectation(
                                        156,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        64,
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        Subtype(
                                            ExpectSubtypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 39,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 159,
                                src: ExpectationSource {
                                    expr_idx: 184,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 39,
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
                                idx: 160,
                                src: ExpectationSource {
                                    expr_idx: 186,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        70,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 161,
                                src: ExpectationSource {
                                    expr_idx: 187,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 162,
                                src: ExpectationSource {
                                    expr_idx: 201,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        71,
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
                                idx: 163,
                                src: ExpectationSource {
                                    expr_idx: 206,
                                    kind: Expr,
                                },
                                expectee: Curry(
                                    EtherealTermCurry(
                                        Id {
                                            value: 18,
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
                                                                72,
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
                                                                76,
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
                                                                            value: 60,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    Keyed(
                                                        FluffyTermRitchieKeyedParameter {
                                                            key: Ident(
                                                                Coword(
                                                                    Id {
                                                                        value: 448,
                                                                    },
                                                                ),
                                                            ),
                                                            contract: None,
                                                            ty: EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 44,
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
                                                        77,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 164,
                                src: ExpectationSource {
                                    expr_idx: 211,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 44,
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 91,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 165,
                                src: ExpectationSource {
                                    expr_idx: 212,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        77,
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
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 99,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 166,
                                src: ExpectationSource {
                                    expr_idx: 212,
                                    kind: Expectation(
                                        165,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        72,
                                    ),
                                ),
                                resolve_progress: Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: EqsExactly(
                                ExpectSubtype {
                                    expected: EntityPath(
                                        TypeVariant(
                                            TypeVariantPath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 167,
                                src: ExpectationSource {
                                    expr_idx: 212,
                                    kind: Expectation(
                                        165,
                                    ),
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        76,
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        Subtype(
                                            ExpectSubtypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 39,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 168,
                                src: ExpectationSource {
                                    expr_idx: 213,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 39,
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
                                idx: 169,
                                src: ExpectationSource {
                                    expr_idx: 215,
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
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 170,
                                src: ExpectationSource {
                                    expr_idx: 216,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 171,
                                src: ExpectationSource {
                                    expr_idx: 217,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 71,
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
                                idx: 172,
                                src: ExpectationSource {
                                    expr_idx: 218,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                idx: 173,
                                src: ExpectationSource {
                                    expr_idx: 219,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 71,
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
                                idx: 174,
                                src: ExpectationSource {
                                    expr_idx: 220,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
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
                                idx: 175,
                                src: ExpectationSource {
                                    expr_idx: 227,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        80,
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
                                idx: 176,
                                src: ExpectationSource {
                                    expr_idx: 229,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        80,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 177,
                                src: ExpectationSource {
                                    expr_idx: 230,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 178,
                                src: ExpectationSource {
                                    expr_idx: 232,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        81,
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 179,
                                src: ExpectationSource {
                                    expr_idx: 233,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 63,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 180,
                                src: ExpectationSource {
                                    expr_idx: 234,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 63,
                                        },
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
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 63,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 181,
                                src: ExpectationSource {
                                    expr_idx: 235,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 63,
                                        },
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
            EtherealTerm(`OneVsAll MnistLabel One`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntitySynNodePath::ModuleItem(
                ModuleItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
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
                        Ok(
                            Solid(
                                SolidTerm(
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
                        MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Place(
                                        StackPure {
                                            location: StackLocationIdx(
                                                LocalSymbolIdx(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ),
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenic_parameters: [],
                                        return_ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 87,
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
                                            value: 87,
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
                                    0,
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
                                            value: 41,
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
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
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
                        data: [
                            TypeOntologyAtPlace {
                                path: TypePath(
                                    Id {
                                        value: 68,
                                    },
                                ),
                                refined_path: Left(
                                    Borrow(
                                        Leash,
                                    ),
                                ),
                                arguments: [
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 93,
                                                },
                                            ),
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 47,
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
                        ],
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
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: Solid(
                                    SolidTerm(
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
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
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
                                idx: 2,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
            EtherealTerm(`Option f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntitySynNodePath::ModuleItem(
                ModuleItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
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
                        Ok(
                            Solid(
                                SolidTerm(
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
                        MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Place(
                                        StackPure {
                                            location: StackLocationIdx(
                                                LocalSymbolIdx(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ),
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenic_parameters: [],
                                        return_ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 87,
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
                                            value: 87,
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
                                    0,
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
                                            value: 41,
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
                        Ok(
                            Solid(
                                SolidTerm(
                                    0,
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
                        MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Place(
                                        StackPure {
                                            location: StackLocationIdx(
                                                LocalSymbolIdx(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ),
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenic_parameters: [],
                                        return_ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 85,
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
                                            value: 85,
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
                        FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [],
                                ty_path: TypePath(
                                    Id {
                                        value: 85,
                                    },
                                ),
                                signature: FluffyFieldSignature {
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
                            },
                        ),
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
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
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
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
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    8,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
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
                        data: [
                            TypeOntologyAtPlace {
                                path: TypePath(
                                    Id {
                                        value: 68,
                                    },
                                ),
                                refined_path: Left(
                                    Borrow(
                                        Leash,
                                    ),
                                ),
                                arguments: [
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 93,
                                                },
                                            ),
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 47,
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
                        ],
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
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: Solid(
                                    SolidTerm(
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
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
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
                                idx: 2,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: Solid(
                                    SolidTerm(
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
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 85,
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
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 35,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            WrapInSome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 35,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 60,
                                            },
                                        ),
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Ok(
                                        ImplicitlyConvertible(
                                            WrapInSome,
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
            EtherealTerm(`Option f32`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntitySynNodePath::ModuleItem(
                ModuleItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
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
                        Ok(
                            Solid(
                                SolidTerm(
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
                        MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Place(
                                        StackPure {
                                            location: StackLocationIdx(
                                                LocalSymbolIdx(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ),
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenic_parameters: [],
                                        return_ty: EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    Id {
                                                        value: 87,
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
                                            value: 87,
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
                                    0,
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
                                            value: 41,
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
                                            value: 41,
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
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    0,
                                ),
                            ),
                        ),
                    ),
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
                        data: [
                            TypeOntologyAtPlace {
                                path: TypePath(
                                    Id {
                                        value: 68,
                                    },
                                ),
                                refined_path: Left(
                                    Borrow(
                                        Leash,
                                    ),
                                ),
                                arguments: [
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 93,
                                                },
                                            ),
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 47,
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
                        ],
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
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: Solid(
                                    SolidTerm(
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
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 87,
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
                                idx: 2,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 41,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 41,
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
            EtherealTerm(`Option f32`),
        ),
        self_ty: None,
    },
]