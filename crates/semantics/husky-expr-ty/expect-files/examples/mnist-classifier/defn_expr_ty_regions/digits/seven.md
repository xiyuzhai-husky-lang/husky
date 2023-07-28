[
    ExprTypeRegion {
        path: RegionPath::Defn(
            EntitySynNodePath::ModuleItem(
                ModuleItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                        value: 45,
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
                        List(
                            NewList,
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
                    entries: [],
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
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Ritchie(
                                    EtherealTermRitchie {
                                        ritchie_kind: FnType,
                                        parameter_contracted_tys: [
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: EtherealTerm(`Leash List ConcaveComponent`),
                                                },
                                            ),
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: EtherealTerm(`List fn( Leash ConcaveComponent) -> Option f32`),
                                                },
                                            ),
                                        ],
                                        return_ty: EtherealTerm(`FermiMatchResult`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                implicit_parameter_substitutions: [],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: Application(
                                                                    EtherealTermApplication(
                                                                        Id {
                                                                            value: 45,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: Application(
                                                                    EtherealTermApplication(
                                                                        Id {
                                                                            value: 47,
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`Leash`),
                                            argument: EtherealTerm(`List ConcaveComponent`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`List ConcaveComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Ritchie(
                                        EtherealTermRitchie {
                                            ritchie_kind: FnType,
                                            parameter_contracted_tys: [
                                                EtherealTermRitchieParameter::Regular(
                                                    EtherealTermRitchieRegularParameter {
                                                        contract: None,
                                                        ty: EtherealTerm(`Leash ConcaveComponent`),
                                                    },
                                                ),
                                            ],
                                            return_ty: EtherealTerm(`Option f32`),
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Ritchie(
                                    EtherealTermRitchie {
                                        ritchie_kind: FnType,
                                        parameter_contracted_tys: [
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: EtherealTerm(`Leash ConcaveComponent`),
                                                },
                                            ),
                                        ],
                                        return_ty: EtherealTerm(`Option f32`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`List`),
                                            argument: EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`List`),
                                        argument: EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
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
                            path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
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
                    entries: [
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::mem::Leash`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Borrow(
                                    Leash,
                                ),
                            ),
                            arguments: [
                                FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            ],
                            base_ty_term: Some(
                                EtherealTerm(`Leash ConcaveComponent`),
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
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
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
                            path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
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
                                        value: 45,
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
                        List(
                            NewList,
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
                    4,
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
                    entries: [],
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
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Ritchie(
                                    EtherealTermRitchie {
                                        ritchie_kind: FnType,
                                        parameter_contracted_tys: [
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: EtherealTerm(`Leash List ConcaveComponent`),
                                                },
                                            ),
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: EtherealTerm(`List fn( Leash ConcaveComponent) -> Option f32`),
                                                },
                                            ),
                                        ],
                                        return_ty: EtherealTerm(`FermiMatchResult`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                implicit_parameter_substitutions: [],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: Application(
                                                                    EtherealTermApplication(
                                                                        Id {
                                                                            value: 45,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: Application(
                                                                    EtherealTermApplication(
                                                                        Id {
                                                                            value: 47,
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`Leash`),
                                            argument: EtherealTerm(`List ConcaveComponent`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`List ConcaveComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Ritchie(
                                        EtherealTermRitchie {
                                            ritchie_kind: FnType,
                                            parameter_contracted_tys: [
                                                EtherealTermRitchieParameter::Regular(
                                                    EtherealTermRitchieRegularParameter {
                                                        contract: None,
                                                        ty: EtherealTerm(`Leash ConcaveComponent`),
                                                    },
                                                ),
                                            ],
                                            return_ty: EtherealTerm(`Option f32`),
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Ritchie(
                                    EtherealTermRitchie {
                                        ritchie_kind: FnType,
                                        parameter_contracted_tys: [
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: EtherealTerm(`Leash ConcaveComponent`),
                                                },
                                            ),
                                        ],
                                        return_ty: EtherealTerm(`Option f32`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Ritchie(
                                        EtherealTermRitchie {
                                            ritchie_kind: FnType,
                                            parameter_contracted_tys: [
                                                EtherealTermRitchieParameter::Regular(
                                                    EtherealTermRitchieRegularParameter {
                                                        contract: None,
                                                        ty: EtherealTerm(`Leash ConcaveComponent`),
                                                    },
                                                ),
                                            ],
                                            return_ty: EtherealTerm(`Option f32`),
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Ritchie(
                                    EtherealTermRitchie {
                                        ritchie_kind: FnType,
                                        parameter_contracted_tys: [
                                            EtherealTermRitchieParameter::Regular(
                                                EtherealTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: EtherealTerm(`Leash ConcaveComponent`),
                                                },
                                            ),
                                        ],
                                        return_ty: EtherealTerm(`Option f32`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`List`),
                                            argument: EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`List`),
                                        argument: EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
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
                            path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
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
                        FieldDispatch(
                            FluffyFieldDispatch {
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
                    5,
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
                                            value: 41,
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
                    9,
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
                    10,
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
                    entries: [
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::mem::Leash`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Borrow(
                                    Leash,
                                ),
                            ),
                            arguments: [
                                FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            ],
                            base_ty_term: Some(
                                EtherealTerm(`Leash ConcaveComponent`),
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
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::f32`, `Extern`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`Option`),
                                            argument: EtherealTerm(`f32`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 13,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            WrapInSome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`Option`),
                                            argument: EtherealTerm(`f32`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 14,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
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
                            path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
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
                        FieldDispatch(
                            FluffyFieldDispatch {
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
                    5,
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
                                            value: 41,
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
                    9,
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
                    10,
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
                    11,
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
                                        parenic_parameters: [
                                            Regular(
                                                FluffyTermRitchieRegularParameter {
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
                solid_terms: SolidTerms {
                    entries: [
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::mem::Leash`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Borrow(
                                    Leash,
                                ),
                            ),
                            arguments: [
                                FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            ],
                            base_ty_term: Some(
                                EtherealTerm(`Leash ConcaveComponent`),
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
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    16,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::f32`, `Extern`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 13,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 14,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 16,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 14,
                                src: ExpectationSource {
                                    expr_idx: 17,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
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
                            path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
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
                                        value: 65,
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
                                        value: 67,
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
                                        value: 70,
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
                                        value: 83,
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
                    9,
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
                    11,
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
                                        value: 83,
                                    },
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
                                                value: 85,
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
                                        value: 85,
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
                    14,
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
                                                    value: 84,
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
                                        value: 84,
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
                    17,
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
                    18,
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
                    20,
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
                    19,
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
                    21,
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
                    22,
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
                                    2,
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
                    24,
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
                                        value: 83,
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
                                                value: 85,
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
                                        value: 85,
                                    },
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
                            Hollow(
                                HollowTerm(
                                    3,
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
                        IndexOrComposeWithList(
                            Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: FluffyIndexSignature {
                                        element_ty: Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 84,
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
                                        value: 84,
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
                        UnwrapOrComposeWithNot(
                            Unwrap,
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
                    30,
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
                    31,
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
                                    4,
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
                disambiguation_and_ty_result: Ok(
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
                                            value: 41,
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
                                        value: 68,
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
                                    5,
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
                                            value: 41,
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
                                        value: 70,
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
                    41,
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
                    42,
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
                                    6,
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
                    44,
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
                    45,
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
                                        value: 83,
                                    },
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
                                                value: 85,
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
                                        value: 85,
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
                    48,
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
                                                    value: 84,
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
                                        value: 84,
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
                    51,
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
                                        value: 83,
                                    },
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
                    53,
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
                    54,
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
                    55,
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
                                        value: 68,
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    56,
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
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::mem::Leash`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Borrow(
                                    Leash,
                                ),
                            ),
                            arguments: [
                                FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`OneVsAll MnistLabel`),
                                        argument: EtherealTerm(`Six`),
                                        shift: 0,
                                    },
                                ),
                            ],
                            base_ty_term: Some(
                                EtherealTerm(`Leash OneVsAll MnistLabel Six`),
                            ),
                            place: StackPure {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        0,
                                    ),
                                ),
                            },
                        },
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::mem::Leash`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Borrow(
                                    Leash,
                                ),
                            ),
                            arguments: [
                                FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`OneVsAll MnistLabel`),
                                        argument: EtherealTerm(`Zero`),
                                        shift: 0,
                                    },
                                ),
                            ],
                            base_ty_term: Some(
                                EtherealTerm(`Leash OneVsAll MnistLabel Zero`),
                            ),
                            place: StackPure {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        1,
                                    ),
                                ),
                            },
                        },
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::option::Option`, `Enum`),
                            refined_path: Left(
                                PreludeTypePath::Option,
                            ),
                            arguments: [
                                FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`ConcaveComponent`),
                                        shift: 0,
                                    },
                                ),
                            ],
                            base_ty_term: Some(
                                EtherealTerm(`Option Leash ConcaveComponent`),
                            ),
                            place: StackPure {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        3,
                                    ),
                                ),
                            },
                        },
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::option::Option`, `Enum`),
                            refined_path: Left(
                                PreludeTypePath::Option,
                            ),
                            arguments: [
                                FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`ConcaveComponent`),
                                        shift: 0,
                                    },
                                ),
                            ],
                            base_ty_term: Some(
                                EtherealTerm(`Option Leash ConcaveComponent`),
                            ),
                            place: StackPure {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        8,
                                    ),
                                ),
                            },
                        },
                    ],
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    11,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    15,
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
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    24,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    28,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    34,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    40,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    48,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    53,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [
                                    CoercibleTo {
                                        target: Hollow(
                                            HollowTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                    ],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`OneVsAll MnistLabel Six`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`OneVsAll MnistLabel Zero`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`ConnectedComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::f32`, `Extern`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`FermiMatchResult`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 13,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`FermiMatchResult`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 14,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`List`),
                                        argument: EtherealTerm(`Option Leash ConcaveComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 14,
                                src: ExpectationSource {
                                    expr_idx: 15,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 16,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 16,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Option`),
                                        argument: EtherealTerm(`Leash ConcaveComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 18,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`ConnectedComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 19,
                                src: ExpectationSource {
                                    expr_idx: 20,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 19,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`ConnectedComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 21,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 22,
                                src: ExpectationSource {
                                    expr_idx: 22,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 23,
                                src: ExpectationSource {
                                    expr_idx: 24,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        2,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 24,
                                src: ExpectationSource {
                                    expr_idx: 25,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 25,
                                src: ExpectationSource {
                                    expr_idx: 26,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`FermiMatchResult`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 26,
                                src: ExpectationSource {
                                    expr_idx: 27,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`List`),
                                        argument: EtherealTerm(`Option Leash ConcaveComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 27,
                                src: ExpectationSource {
                                    expr_idx: 28,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        3,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 28,
                                src: ExpectationSource {
                                    expr_idx: 29,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        3,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 29,
                                src: ExpectationSource {
                                    expr_idx: 29,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Option`),
                                        argument: EtherealTerm(`Leash ConcaveComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 30,
                                src: ExpectationSource {
                                    expr_idx: 30,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`ConcaveComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 31,
                                src: ExpectationSource {
                                    expr_idx: 31,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 32,
                                src: ExpectationSource {
                                    expr_idx: 34,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        4,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 33,
                                src: ExpectationSource {
                                    expr_idx: 36,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        4,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 34,
                                src: ExpectationSource {
                                    expr_idx: 37,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`OneVsAll MnistLabel`),
                                            argument: EtherealTerm(`Seven`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 35,
                                src: ExpectationSource {
                                    expr_idx: 38,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`OneVsAll MnistLabel`),
                                        argument: EtherealTerm(`Seven`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 36,
                                src: ExpectationSource {
                                    expr_idx: 40,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        5,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 37,
                                src: ExpectationSource {
                                    expr_idx: 41,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 38,
                                src: ExpectationSource {
                                    expr_idx: 42,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`ConnectedComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 39,
                                src: ExpectationSource {
                                    expr_idx: 44,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 40,
                                src: ExpectationSource {
                                    expr_idx: 43,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`ConnectedComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 41,
                                src: ExpectationSource {
                                    expr_idx: 45,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 42,
                                src: ExpectationSource {
                                    expr_idx: 46,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 43,
                                src: ExpectationSource {
                                    expr_idx: 48,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        6,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 44,
                                src: ExpectationSource {
                                    expr_idx: 49,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`OneVsAll MnistLabel`),
                                            argument: EtherealTerm(`Seven`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 45,
                                src: ExpectationSource {
                                    expr_idx: 50,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`OneVsAll MnistLabel`),
                                        argument: EtherealTerm(`Seven`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 46,
                                src: ExpectationSource {
                                    expr_idx: 51,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`FermiMatchResult`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 47,
                                src: ExpectationSource {
                                    expr_idx: 52,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`List`),
                                        argument: EtherealTerm(`Option Leash ConcaveComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 48,
                                src: ExpectationSource {
                                    expr_idx: 53,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        7,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 49,
                                src: ExpectationSource {
                                    expr_idx: 54,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        7,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Holed,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 50,
                                src: ExpectationSource {
                                    expr_idx: 54,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Option`),
                                        argument: EtherealTerm(`Leash ConcaveComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 51,
                                src: ExpectationSource {
                                    expr_idx: 55,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 52,
                                src: ExpectationSource {
                                    expr_idx: 56,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Leash`),
                                        argument: EtherealTerm(`FermiMatchResult`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 53,
                                src: ExpectationSource {
                                    expr_idx: 57,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`List`),
                                        argument: EtherealTerm(`Leash ConcaveComponent`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 54,
                                src: ExpectationSource {
                                    expr_idx: 58,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`OneVsAll MnistLabel`),
                                            argument: EtherealTerm(`Seven`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 55,
                                src: ExpectationSource {
                                    expr_idx: 59,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`OneVsAll MnistLabel`),
                                        argument: EtherealTerm(`Seven`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Application(
                                        EtherealTermApplication {
                                            function: EtherealTerm(`OneVsAll MnistLabel`),
                                            argument: EtherealTerm(`Seven`),
                                            shift: 0,
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 56,
                                src: ExpectationSource {
                                    expr_idx: 60,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`OneVsAll MnistLabel`),
                                        argument: EtherealTerm(`Seven`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
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
            EtherealTerm(`OneVsAll MnistLabel Seven`),
        ),
        self_ty: None,
    },
]