[
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
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
                                        value: 11,
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
                    3,
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
                                            value: 96,
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
                                            value: 12,
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
                                    ty: Application(
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
                                    ty: Ritchie(
                                        EtherealTermRitchie(
                                            Id {
                                                value: 11,
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
                                            value: 11,
                                        },
                                    ),
                                ),
                                resolve_progress: Resolved(
                                    Err(
                                        Original(
                                            ExpectedCoersion,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
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
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
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
                                    ty: EntityPath(
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
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
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
                                    ty: EntityPath(
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
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
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
            EtherealTerm(`TypeOntology(mnist_classifier::fermi::FermiMatchResult)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
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
                                        value: 64,
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
                                            value: 41,
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
                                        value: 66,
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 68,
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
                        TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 70,
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
                    7,
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
                    10,
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
                    9,
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
                                            value: 60,
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
                                        value: 71,
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
                        Field(
                            FluffyFieldDisambiguation {
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
                                                    value: 80,
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
                                            value: 80,
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
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [],
                                ty_path: TypePath(
                                    Id {
                                        value: 80,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 40,
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
                                        value: 40,
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
                    16,
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
                                                    value: 39,
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
                                        value: 39,
                                    },
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
                    19,
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
                    20,
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
                                        value: 81,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 80,
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
                                            value: 80,
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
                        Field(
                            FluffyFieldDisambiguation {
                                indirections: [],
                                ty_path: TypePath(
                                    Id {
                                        value: 80,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 40,
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
                                        value: 40,
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
                    23,
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
                                                    value: 39,
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
                                        value: 39,
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
                    27,
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
                    28,
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
                                        value: 50,
                                    },
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
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 50,
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
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    2,
                                ),
                            ),
                        ),
                    ),
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    3,
                                ),
                            ),
                        ),
                    ),
                    None,
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    4,
                                ),
                            ),
                        ),
                    ),
                    Some(
                        SymbolType(
                            Solid(
                                SolidTerm(
                                    5,
                                ),
                            ),
                        ),
                    ),
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
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 63,
                                            },
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 64,
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
                                        value: 68,
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
                                                value: 65,
                                            },
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 66,
                                            },
                                        ),
                                    ),
                                ),
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            1,
                                        ),
                                    ),
                                },
                            },
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
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 67,
                                            },
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 68,
                                            },
                                        ),
                                    ),
                                ),
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            2,
                                        ),
                                    ),
                                },
                            },
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
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 69,
                                            },
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 70,
                                            },
                                        ),
                                    ),
                                ),
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            3,
                                        ),
                                    ),
                                },
                            },
                            TypeOntologyAtPlace {
                                path: TypePath(
                                    Id {
                                        value: 37,
                                    },
                                ),
                                refined_path: Left(
                                    Option,
                                ),
                                arguments: [
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 33,
                                            },
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 39,
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
                                        value: 37,
                                    },
                                ),
                                refined_path: Left(
                                    Option,
                                ),
                                arguments: [
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 33,
                                            },
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 39,
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
                                    16,
                                ),
                                hole_kind: UnspecifiedIntegerType,
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
                            data: Hole {
                                hole_source: Expr(
                                    22,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
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
                                            value: 64,
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
                                    ty: EntityPath(
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
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
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
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 66,
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
                                    ty: EntityPath(
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
                                    expr_idx: 3,
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
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 68,
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
                                    ty: EntityPath(
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
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 70,
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
                                    ty: EntityPath(
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
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
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
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 8,
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
                                idx: 9,
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
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 9,
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
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 11,
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
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 12,
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
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 13,
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
                                idx: 14,
                                src: ExpectationSource {
                                    expr_idx: 14,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 80,
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
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 15,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 40,
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
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 16,
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
                                    ty: EntityPath(
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
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 17,
                                    kind: Expr,
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
                            expectation: AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 17,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 39,
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
                                    ty: EntityPath(
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
                                idx: 19,
                                src: ExpectationSource {
                                    expr_idx: 18,
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
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 19,
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
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 20,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 80,
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
                                idx: 22,
                                src: ExpectationSource {
                                    expr_idx: 21,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 40,
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
                                idx: 23,
                                src: ExpectationSource {
                                    expr_idx: 22,
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
                                    ty: EntityPath(
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
                                idx: 24,
                                src: ExpectationSource {
                                    expr_idx: 23,
                                    kind: Expr,
                                },
                                expectee: Hollow(
                                    HollowTerm(
                                        1,
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
                                idx: 25,
                                src: ExpectationSource {
                                    expr_idx: 23,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 39,
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
                                    ty: EntityPath(
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
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty: EntityPath(
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
                                idx: 27,
                                src: ExpectationSource {
                                    expr_idx: 25,
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
                                    contract: None,
                                    ty: EntityPath(
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
                                idx: 28,
                                src: ExpectationSource {
                                    expr_idx: 26,
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
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 50,
                                            },
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
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 50,
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
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 50,
                                            },
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
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 50,
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
            EtherealTerm(`TypeOntology(malamute::OneVsAll) TypeOntology(mnist::MnistLabel) TypeVariant(mnist::MnistLabel::Eight)`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
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
                        Field(
                            FluffyFieldDisambiguation {
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
                                                    value: 90,
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
                                            value: 90,
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
                                            value: 41,
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
                            Solid(
                                SolidTerm(
                                    0,
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
                        Field(
                            FluffyFieldDisambiguation {
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
                                ty_path: TypePath(
                                    Id {
                                        value: 93,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 73,
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
                                        value: 73,
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
                    Original(
                        NoMethodForType {
                            self_expr_ty: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 73,
                                    },
                                ),
                            ),
                            ident_token: IdentToken {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 42,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    115,
                                ),
                            },
                        },
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        UnableToInferSuffixOperandType,
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
                            Solid(
                                SolidTerm(
                                    0,
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
                        Field(
                            FluffyFieldDisambiguation {
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
                                ty_path: TypePath(
                                    Id {
                                        value: 93,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 73,
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
                                        value: 73,
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
                disambiguation_and_ty_result: Err(
                    Original(
                        NoMethodForType {
                            self_expr_ty: Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 73,
                                    },
                                ),
                            ),
                            ident_token: IdentToken {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 42,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    128,
                                ),
                            },
                        },
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    Derived(
                        UnableToInferSuffixOperandType,
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
                    9,
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
                    10,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Field(
                            FluffyFieldDisambiguation {
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
                                                    value: 90,
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
                                            value: 90,
                                        },
                                    ),
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
                    12,
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
                    13,
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
                    entries: [],
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
                                                value: 90,
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
                                resolve_progress: Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: ImplicitlyConvertible(
                                ExpectCoersion {
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
                                    ty: EntityPath(
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
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
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
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
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
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 73,
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
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 10,
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
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 73,
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
                                    ty: EntityPath(
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
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 17,
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
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 18,
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
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 19,
                                    kind: Expr,
                                },
                                expectee: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 90,
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
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 20,
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
                                    ty: Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 21,
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
            EtherealTerm(`TypeOntology(core::option::Option) TypeOntology(core::num::f32)`),
        ),
        self_ty: None,
    },
]