[
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`fn( Leash Vec ConcaveComponent,  Vec fn( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                ),
                            },
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash Vec ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                ),
                            },
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                ),
                            },
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                ),
                            },
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
                        ExprDisambiguation::List(
                            ListExprDisambiguation::NewList,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Vec fn( Leash ConcaveComponent) -> Option f32`),
                                ),
                            },
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
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::FnCall {
                                ritchie_parameter_argument_matches: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 31,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        RegularOrVariadicCallListItem {
                                            argument_expr_idx: 1,
                                            separator: Comma(
                                                TokenIdx(
                                                    12,
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 33,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        RegularOrVariadicCallListItem {
                                            argument_expr_idx: 5,
                                            separator: None,
                                        },
                                    ),
                                ],
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`FermiMatchResult`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`FermiMatchResult`),
                                ),
                            },
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
            inherited_symbol_map: [],
            current_symbol_map: [],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [],
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`fn( Leash Vec ConcaveComponent,  Vec fn( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`FermiMatchResult`),
                                                    ),
                                                },
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        Application(
                                                                            EtherealTermApplication(
                                                                                Id {
                                                                                    value: 31,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        Application(
                                                                            EtherealTermApplication(
                                                                                Id {
                                                                                    value: 33,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
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
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`fn( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec fn( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec fn( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`FermiMatchResult`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 56,
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            },
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 56,
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            },
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 56,
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            },
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 56,
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            },
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 56,
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            },
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 49,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 49,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
        ],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
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
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
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
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
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
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
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
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
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
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 49,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 49,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash Vec ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [],
                                        return_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 15,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`i32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`i32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash Vec ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [],
                                        return_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 15,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`i32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`i32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash FermiMatchResult`),
                                ),
                            },
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
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 60,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 57,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                            },
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
                        ExprDisambiguation::IndexOrComposeWithList(
                            IndexOrComposeWithListExprDisambiguation::Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: Int {
                                        element_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 56,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            ),
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Option Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash FermiMatchResult`),
                                ),
                            },
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
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 60,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 57,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                            },
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
                        ExprDisambiguation::IndexOrComposeWithList(
                            IndexOrComposeWithListExprDisambiguation::Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: Int {
                                        element_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 56,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            ),
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Option Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash FermiMatchResult`),
                                ),
                            },
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
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 60,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 57,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Hollow(
                                    HollowTerm(
                                        2,
                                    ),
                                ),
                            },
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
                        ExprDisambiguation::IndexOrComposeWithList(
                            IndexOrComposeWithListExprDisambiguation::Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: Int {
                                        element_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 56,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                },
                            ),
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Option Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Option Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Option Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::UnwrapOrComposeWithNot(
                            UnwrapOrComposeWithNotExprDisambiguation::Unwrap,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 57,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Option Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Option Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::UnwrapOrComposeWithNot(
                            UnwrapOrComposeWithNotExprDisambiguation::Unwrap,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [],
                                        return_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
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
                                        },
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Vector2d`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [
                                            Regular(
                                                FluffyTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: Ethereal(
                                                            EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 2,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ),
                                        ],
                                        return_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 25,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Hollow(
                                    HollowTerm(
                                        3,
                                    ),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Hollow(
                                    HollowTerm(
                                        3,
                                    ),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Option Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::UnwrapOrComposeWithNot(
                            UnwrapOrComposeWithNotExprDisambiguation::Unwrap,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [],
                                        return_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 49,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Point2d`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Option Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::UnwrapOrComposeWithNot(
                            UnwrapOrComposeWithNotExprDisambiguation::Unwrap,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [],
                                        return_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 49,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Point2d`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Point2d`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Point2d`),
                                ),
                            },
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
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [
                                            Regular(
                                                FluffyTermRitchieRegularParameter {
                                                    contract: None,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: Ethereal(
                                                            EntityPath(
                                                                TypeOntology(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 49,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ),
                                        ],
                                        return_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 25,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash FermiMatchResult`),
                                ),
                            },
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
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 60,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    60,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Option Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::UnwrapOrComposeWithNot(
                            UnwrapOrComposeWithNotExprDisambiguation::Unwrap,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Hollow(
                                    HollowTerm(
                                        4,
                                    ),
                                ),
                            },
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
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 57,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Hollow(
                                    HollowTerm(
                                        4,
                                    ),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`OneVsAll MnistLabel Three`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`OneVsAll MnistLabel Three`),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    68,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`2`),
                    ),
                },
            ),
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`4`),
                    ),
                },
            ),
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`0`),
                    ),
                },
            ),
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`1`),
                    ),
                },
            ),
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`2`),
                    ),
                },
            ),
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`3`),
                    ),
                },
            ),
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`true`),
                    ),
                },
            ),
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`0`),
                    ),
                },
            ),
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`100`),
                    ),
                },
            ),
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`20`),
                    ),
                },
            ),
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`2.5`),
                    ),
                },
            ),
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`100`),
                    ),
                },
            ),
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Option Leash ConcaveComponent`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Option Leash ConcaveComponent`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Option Leash ConcaveComponent`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Option Leash ConcaveComponent`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Option Leash ConcaveComponent`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`f32`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Point2d`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Point2d`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`f32`),
                        ),
                    },
                ),
            ],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [],
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    10,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 24,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleInto {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 24,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`usize`),
                            ),
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    14,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 24,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleInto {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 24,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`usize`),
                            ),
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    18,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 24,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleInto {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 24,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`usize`),
                            ),
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    36,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleInto {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 25,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`f32`),
                            ),
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    60,
                                ),
                                hole_kind: UnspecifiedFloatType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleInto {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 25,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`f32`),
                            ),
                        },
                    ],
                    first_unresolved_term_idx: 5,
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`i32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash FermiMatchResult`),
                                    ),
                                },
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
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
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
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
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash FermiMatchResult`),
                                    ),
                                },
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
                                    expr_idx: 13,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 14,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 15,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 15,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 16,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash FermiMatchResult`),
                                    ),
                                },
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
                                    expr_idx: 17,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
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
                                    expr_idx: 18,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 19,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                                    expr_idx: 19,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 23,
                                src: ExpectationSource {
                                    expr_idx: 20,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 24,
                                src: ExpectationSource {
                                    expr_idx: 21,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 25,
                                src: ExpectationSource {
                                    expr_idx: 22,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
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
                                    expr_idx: 23,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
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
                                    expr_idx: 24,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 28,
                                src: ExpectationSource {
                                    expr_idx: 25,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 29,
                                src: ExpectationSource {
                                    expr_idx: 26,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
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
                                idx: 30,
                                src: ExpectationSource {
                                    expr_idx: 27,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 31,
                                src: ExpectationSource {
                                    expr_idx: 28,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 32,
                                src: ExpectationSource {
                                    expr_idx: 29,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 33,
                                src: ExpectationSource {
                                    expr_idx: 30,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 34,
                                src: ExpectationSource {
                                    expr_idx: 31,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 35,
                                src: ExpectationSource {
                                    expr_idx: 32,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                                idx: 36,
                                src: ExpectationSource {
                                    expr_idx: 33,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 37,
                                src: ExpectationSource {
                                    expr_idx: 34,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 38,
                                src: ExpectationSource {
                                    expr_idx: 35,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 39,
                                src: ExpectationSource {
                                    expr_idx: 39,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                                idx: 40,
                                src: ExpectationSource {
                                    expr_idx: 37,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 41,
                                src: ExpectationSource {
                                    expr_idx: 36,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            3,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 42,
                                src: ExpectationSource {
                                    expr_idx: 38,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            3,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`bool`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 43,
                                src: ExpectationSource {
                                    expr_idx: 40,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 44,
                                src: ExpectationSource {
                                    expr_idx: 41,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 45,
                                src: ExpectationSource {
                                    expr_idx: 42,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 46,
                                src: ExpectationSource {
                                    expr_idx: 43,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
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
                                    expr_idx: 44,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Point2d`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 48,
                                src: ExpectationSource {
                                    expr_idx: 45,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 49,
                                src: ExpectationSource {
                                    expr_idx: 46,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 50,
                                src: ExpectationSource {
                                    expr_idx: 47,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Point2d`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 51,
                                src: ExpectationSource {
                                    expr_idx: 48,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Point2d`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Point2d`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 52,
                                src: ExpectationSource {
                                    expr_idx: 49,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Point2d`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
                                idx: 53,
                                src: ExpectationSource {
                                    expr_idx: 50,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 54,
                                src: ExpectationSource {
                                    expr_idx: 51,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 55,
                                src: ExpectationSource {
                                    expr_idx: 52,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 56,
                                src: ExpectationSource {
                                    expr_idx: 53,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
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
                                idx: 57,
                                src: ExpectationSource {
                                    expr_idx: 54,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash FermiMatchResult`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 58,
                                src: ExpectationSource {
                                    expr_idx: 55,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 59,
                                src: ExpectationSource {
                                    expr_idx: 56,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 60,
                                src: ExpectationSource {
                                    expr_idx: 57,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::FinalDestination(
                                ExpectFinalDestination {
                                    final_destination: AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 61,
                                src: ExpectationSource {
                                    expr_idx: 58,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 62,
                                src: ExpectationSource {
                                    expr_idx: 59,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 63,
                                src: ExpectationSource {
                                    expr_idx: 61,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 64,
                                src: ExpectationSource {
                                    expr_idx: 60,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            4,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 65,
                                src: ExpectationSource {
                                    expr_idx: 62,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            4,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 66,
                                src: ExpectationSource {
                                    expr_idx: 63,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`OneVsAll MnistLabel Three`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 67,
                                src: ExpectationSource {
                                    expr_idx: 64,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`OneVsAll MnistLabel Three`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`OneVsAll MnistLabel Three`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 68,
                                src: ExpectationSource {
                                    expr_idx: 65,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`OneVsAll MnistLabel Three`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
            EtherealTerm(`OneVsAll MnistLabel Three`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::three::uparc`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
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
                    },
                ),
            },
        ],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
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
                            },
                        ),
                    },
                ),
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [],
                                        return_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
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
                                        },
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Vector2d`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Vector2d`),
                                ),
                            },
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
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [],
                                ty_path: TypePath(
                                    Id {
                                        value: 51,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`(independent variable_ad_hoc_fmt: Type) -> fn(move  variable_ad_hoc_fmt) -> Option variable_ad_hoc_fmt`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 57,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`BoundingBox`),
                                ),
                            },
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
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [],
                                        return_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 25,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::FnCall {
                                ritchie_parameter_argument_matches: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        },
                                        RegularOrVariadicCallListItem {
                                            argument_expr_idx: 10,
                                            separator: None,
                                        },
                                    ),
                                ],
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    12,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`0`),
                    ),
                },
            ),
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash ConcaveComponent`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Vector2d`),
                        ),
                    },
                ),
            ],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [],
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    6,
                                ),
                                hole_kind: ImplicitType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleFrom {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 25,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`f32`),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 28,
                                    },
                                ),
                                refined_path: Left(
                                    Option,
                                ),
                                arguments: [
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`Option f32`),
                            ),
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                ],
                                return_ty: FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`fn(move  f32) -> Option f32`),
                            ),
                        },
                    ],
                    first_unresolved_term_idx: 1,
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`(independent variable_ad_hoc_fmt: Type) -> fn(move  variable_ad_hoc_fmt) -> Option variable_ad_hoc_fmt`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`variable_ad_hoc_fmt`),
                                                            ),
                                                        },
                                                        substitute: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Hollow(
                                                                HollowTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Hollow(
                                                        HollowTerm(
                                                            1,
                                                        ),
                                                    ),
                                                },
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: FluffyTerm {
                                                                    place: None,
                                                                    base: Hollow(
                                                                        HollowTerm(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                },
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
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`BoundingBox`),
                                    ),
                                },
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::three::downarc`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
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
                    },
                ),
            },
        ],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
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
                            },
                        ),
                    },
                ),
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [],
                                        return_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
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
                                        },
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Vector2d`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Vector2d`),
                                ),
                            },
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
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [],
                                ty_path: TypePath(
                                    Id {
                                        value: 51,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`(independent variable_ad_hoc_fmt: Type) -> fn(move  variable_ad_hoc_fmt) -> Option variable_ad_hoc_fmt`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 57,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`BoundingBox`),
                                ),
                            },
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
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [],
                                        return_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 25,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::FnCall {
                                ritchie_parameter_argument_matches: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        },
                                        RegularOrVariadicCallListItem {
                                            argument_expr_idx: 10,
                                            separator: None,
                                        },
                                    ),
                                ],
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    12,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`0`),
                    ),
                },
            ),
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash ConcaveComponent`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Vector2d`),
                        ),
                    },
                ),
            ],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [],
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    6,
                                ),
                                hole_kind: ImplicitType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleFrom {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 25,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`f32`),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 28,
                                    },
                                ),
                                refined_path: Left(
                                    Option,
                                ),
                                arguments: [
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`Option f32`),
                            ),
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                ],
                                return_ty: FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`fn(move  f32) -> Option f32`),
                            ),
                        },
                    ],
                    first_unresolved_term_idx: 1,
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`(independent variable_ad_hoc_fmt: Type) -> fn(move  variable_ad_hoc_fmt) -> Option variable_ad_hoc_fmt`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`variable_ad_hoc_fmt`),
                                                            ),
                                                        },
                                                        substitute: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Hollow(
                                                                HollowTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Hollow(
                                                        HollowTerm(
                                                            1,
                                                        ),
                                                    ),
                                                },
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: FluffyTerm {
                                                                    place: None,
                                                                    base: Hollow(
                                                                        HollowTerm(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                },
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
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`BoundingBox`),
                                    ),
                                },
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`mnist_classifier::digits::three::back`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
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
                    },
                ),
            },
        ],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                Some(
                    PatternSymbolTypeInfo {
                        ty: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
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
                            },
                        ),
                    },
                ),
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [
                                    Leash,
                                ],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [],
                                        return_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
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
                                        },
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Vector2d`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Vector2d`),
                                ),
                            },
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
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [],
                                ty_path: TypePath(
                                    Id {
                                        value: 51,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`bool`),
                                ),
                            },
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`(independent variable_ad_hoc_fmt: Type) -> fn(move  variable_ad_hoc_fmt) -> Option variable_ad_hoc_fmt`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`Leash ConcaveComponent`),
                                ),
                            },
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
                        ExprDisambiguation::FieldDispatch(
                            FluffyFieldDispatch {
                                indirections: [
                                    Leash,
                                ],
                                ty_path: TypePath(
                                    Id {
                                        value: 57,
                                    },
                                ),
                                signature: FluffyFieldSignature {
                                    ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 53,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`BoundingBox`),
                                ),
                            },
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
                        ExprDisambiguation::MethodDispatch(
                            FluffyDynamicDispatch {
                                indirections: [],
                                signature: MethodFn(
                                    MethodFnFluffySignature {
                                        parenate_parameters: [],
                                        return_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 25,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Ethereal(
                                    EtherealTerm(`f32`),
                                ),
                            },
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
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::FnCall {
                                ritchie_parameter_argument_matches: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        },
                                        RegularOrVariadicCallListItem {
                                            argument_expr_idx: 10,
                                            separator: None,
                                        },
                                    ),
                                ],
                            },
                        ),
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                            },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm {
                                place: None,
                                base: FluffyTermBase::Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    12,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [
            Ok(
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`0`),
                    ),
                },
            ),
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Leash ConcaveComponent`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Vector2d`),
                        ),
                    },
                ),
            ],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [],
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    6,
                                ),
                                hole_kind: ImplicitType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [
                                    CoercibleFrom {
                                        target: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            Id {
                                                                value: 25,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`f32`),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 28,
                                    },
                                ),
                                refined_path: Left(
                                    Option,
                                ),
                                arguments: [
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`Option f32`),
                            ),
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                ],
                                return_ty: FluffyTerm {
                                    place: None,
                                    base: Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`fn(move  f32) -> Option f32`),
                            ),
                        },
                    ],
                    first_unresolved_term_idx: 1,
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vector2d`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`bool`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`(independent variable_ad_hoc_fmt: Type) -> fn(move  variable_ad_hoc_fmt) -> Option variable_ad_hoc_fmt`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Ethereal(
                                                                EtherealTerm(`variable_ad_hoc_fmt`),
                                                            ),
                                                        },
                                                        substitute: FluffyTerm {
                                                            place: None,
                                                            base: FluffyTermBase::Hollow(
                                                                HollowTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Hollow(
                                                        HollowTerm(
                                                            1,
                                                        ),
                                                    ),
                                                },
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: FluffyTerm {
                                                                    place: None,
                                                                    base: Hollow(
                                                                        HollowTerm(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                },
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
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`BoundingBox`),
                                    ),
                                },
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`f32`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Option f32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::ImplicitlyConvertible(
                                            Trivial(
                                                Todo,
                                            ),
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