[
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
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
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Original(
                        OriginalExprTypeError::NoMethodForType {
                            self_expr_ty: FluffyTerm::Solid(
                                SolidTerm(
                                    0,
                                ),
                            ),
                            ident_token: IdentToken {
                                ident: `len`,
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                        },
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Implicit,
                                    variance: Independent,
                                    parameter_variable: Some(
                                        EtherealTermVariable {
                                            ty: EtherealTerm(`Type`),
                                            idx: 0,
                                        },
                                    ),
                                    parameter_ty: EtherealTerm(`Type`),
                                    return_ty: EtherealTerm(`fn(move  Slice variable_ad_hoc_fmt,  isize,  isize) -> unit`),
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::num::isize`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    3,
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BracketedItemTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm::Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::num::isize`, `Extern`),
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
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
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
        expr_fluffy_terms: [
            Ok(
                FluffyTerm::EntityPath(
                    TermEntityPath::TypeOntology(
                        TypePath(`core::num::isize`, `Extern`),
                    ),
                ),
            ),
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    Some(
                        SymbolType(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
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
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                    ),
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
                            path: TypePath(`core::slice::Slice`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Slice,
                            ),
                            arguments: [
                                FluffyTerm::Symbol(
                                    EtherealTermSymbol {
                                        ty: EtherealTerm(`Type`),
                                        index: EtherealTermSymbolIndex(
                                            Type {
                                                attrs: EtherealTemplateSymbolAttrs {
                                                    phantom: false,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                    },
                                ),
                            ],
                            base_ty_term: Some(
                                EtherealTerm(`Slice t`),
                            ),
                            place: MutableStackOwned {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        1,
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
                                hole_source: Expectation(
                                    1,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 32,
                                    },
                                ),
                                refined_path: Left(
                                    Slice,
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: Hollow(
                                                HollowTerm(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 19,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 19,
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
                                                value: 4,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    6,
                                ),
                                hole_kind: UnspecifiedIntegerType,
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
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::AnyDerived,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Implicit,
                                        variance: Independent,
                                        parameter_variable: Some(
                                            EtherealTermVariable {
                                                ty: EtherealTerm(`Type`),
                                                idx: 0,
                                            },
                                        ),
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`fn(move  Slice variable_ad_hoc_fmt,  isize,  isize) -> unit`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: FluffyTerm::Variable(
                                                            EtherealTermVariable {
                                                                ty: EtherealTerm(`Type`),
                                                                idx: 0,
                                                            },
                                                        ),
                                                        substitute: FluffyTerm::Hollow(
                                                            HollowTerm(
                                                                0,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: Hollow(
                                                                    HollowTerm(
                                                                        1,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 19,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 19,
                                                                            },
                                                                        ),
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
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Solid(
                                                    SolidTerm(
                                                        0,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        1,
                                                    ),
                                                ),
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
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::isize`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::isize`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        0,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
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
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 6,
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
                                            TypePath(`core::num::isize`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::isize`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
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
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
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
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
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
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
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
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    2,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Implicit,
                                    variance: Independent,
                                    parameter_variable: Some(
                                        EtherealTermVariable {
                                            ty: EtherealTerm(`Type`),
                                            idx: 0,
                                        },
                                    ),
                                    parameter_ty: EtherealTerm(`Type`),
                                    return_ty: EtherealTerm(`fn(move  Slice variable_ad_hoc_fmt,  isize,  isize) -> isize`),
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    1,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    2,
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
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::num::isize`, `Extern`),
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Implicit,
                                    variance: Independent,
                                    parameter_variable: Some(
                                        EtherealTermVariable {
                                            ty: EtherealTerm(`Type`),
                                            idx: 0,
                                        },
                                    ),
                                    parameter_ty: EtherealTerm(`Type`),
                                    return_ty: EtherealTerm(`fn(move  Slice variable_ad_hoc_fmt,  isize,  isize) -> unit`),
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    1,
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    6,
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Implicit,
                                    variance: Independent,
                                    parameter_variable: Some(
                                        EtherealTermVariable {
                                            ty: EtherealTerm(`Type`),
                                            idx: 0,
                                        },
                                    ),
                                    parameter_ty: EtherealTerm(`Type`),
                                    return_ty: EtherealTerm(`fn(move  Slice variable_ad_hoc_fmt,  isize,  isize) -> unit`),
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    0,
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    10,
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
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
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    18,
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
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
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
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                    ),
                    None,
                    None,
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
                            path: TypePath(`core::slice::Slice`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Slice,
                            ),
                            arguments: [
                                FluffyTerm::Symbol(
                                    EtherealTermSymbol {
                                        ty: EtherealTerm(`Type`),
                                        index: EtherealTermSymbolIndex(
                                            Type {
                                                attrs: EtherealTemplateSymbolAttrs {
                                                    phantom: false,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                    },
                                ),
                            ],
                            base_ty_term: Some(
                                EtherealTerm(`Slice t`),
                            ),
                            place: MutableStackOwned {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        1,
                                    ),
                                ),
                            },
                        },
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::num::isize`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Num(
                                    Int(
                                        ISize,
                                    ),
                                ),
                            ),
                            arguments: [],
                            base_ty_term: Some(
                                EtherealTerm(`isize`),
                            ),
                            place: StackPure {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        2,
                                    ),
                                ),
                            },
                        },
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::num::isize`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Num(
                                    Int(
                                        ISize,
                                    ),
                                ),
                            ),
                            arguments: [],
                            base_ty_term: Some(
                                EtherealTerm(`isize`),
                            ),
                            place: StackPure {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        3,
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
                                hole_source: Expectation(
                                    3,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 32,
                                    },
                                ),
                                refined_path: Left(
                                    Slice,
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: Hollow(
                                                HollowTerm(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 19,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 19,
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
                                                value: 19,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    8,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 32,
                                    },
                                ),
                                refined_path: Left(
                                    Slice,
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            3,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: Hollow(
                                                HollowTerm(
                                                    4,
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 19,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 19,
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
                                                value: 4,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    12,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    13,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 32,
                                    },
                                ),
                                refined_path: Left(
                                    Slice,
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            7,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: Hollow(
                                                HollowTerm(
                                                    8,
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 19,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: None,
                                            ty: EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        Id {
                                                            value: 19,
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
                                                value: 4,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    18,
                                ),
                                hole_kind: UnspecifiedIntegerType,
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
                                            TypePath(`core::num::isize`, `Extern`),
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
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        2,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Solid(
                                                    SolidTerm(
                                                        2,
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
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
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Implicit,
                                        variance: Independent,
                                        parameter_variable: Some(
                                            EtherealTermVariable {
                                                ty: EtherealTerm(`Type`),
                                                idx: 0,
                                            },
                                        ),
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`fn(move  Slice variable_ad_hoc_fmt,  isize,  isize) -> isize`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: FluffyTerm::Variable(
                                                            EtherealTermVariable {
                                                                ty: EtherealTerm(`Type`),
                                                                idx: 0,
                                                            },
                                                        ),
                                                        substitute: FluffyTerm::Hollow(
                                                            HollowTerm(
                                                                0,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: Hollow(
                                                                    HollowTerm(
                                                                        1,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 19,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 19,
                                                                            },
                                                                        ),
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
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            1,
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
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Solid(
                                                    SolidTerm(
                                                        0,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        1,
                                                    ),
                                                ),
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
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::isize`, `Extern`),
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
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        1,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Solid(
                                                    SolidTerm(
                                                        1,
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
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
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::isize`, `Extern`),
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
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        2,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Solid(
                                                    SolidTerm(
                                                        2,
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
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
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::isize`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Implicit,
                                        variance: Independent,
                                        parameter_variable: Some(
                                            EtherealTermVariable {
                                                ty: EtherealTerm(`Type`),
                                                idx: 0,
                                            },
                                        ),
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`fn(move  Slice variable_ad_hoc_fmt,  isize,  isize) -> unit`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: FluffyTerm::Variable(
                                                            EtherealTermVariable {
                                                                ty: EtherealTerm(`Type`),
                                                                idx: 0,
                                                            },
                                                        ),
                                                        substitute: FluffyTerm::Hollow(
                                                            HollowTerm(
                                                                3,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: Hollow(
                                                                    HollowTerm(
                                                                        4,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 19,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 19,
                                                                            },
                                                                        ),
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
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Solid(
                                                    SolidTerm(
                                                        0,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        4,
                                                    ),
                                                ),
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
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::isize`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        1,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Solid(
                                                    SolidTerm(
                                                        1,
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
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
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 12,
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
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
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
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::AnyDerived,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 15,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Implicit,
                                        variance: Independent,
                                        parameter_variable: Some(
                                            EtherealTermVariable {
                                                ty: EtherealTerm(`Type`),
                                                idx: 0,
                                            },
                                        ),
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`fn(move  Slice variable_ad_hoc_fmt,  isize,  isize) -> unit`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: FluffyTerm::Variable(
                                                            EtherealTermVariable {
                                                                ty: EtherealTerm(`Type`),
                                                                idx: 0,
                                                            },
                                                        ),
                                                        substitute: FluffyTerm::Hollow(
                                                            HollowTerm(
                                                                7,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: Hollow(
                                                                    HollowTerm(
                                                                        8,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 19,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: None,
                                                                ty: EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 19,
                                                                            },
                                                                        ),
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
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            8,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 14,
                                src: ExpectationSource {
                                    expr_idx: 16,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        0,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Solid(
                                                    SolidTerm(
                                                        0,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        8,
                                                    ),
                                                ),
                                            },
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
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 18,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        10,
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
                                            TypePath(`core::num::isize`, `Extern`),
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
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        2,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Solid(
                                                    SolidTerm(
                                                        2,
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
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
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 21,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
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
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 22,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
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
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::partition`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    2,
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
                        ExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm::Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::num::usize`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    1,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::num::isize`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::num::isize`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    2,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm::Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::num::usize`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    0,
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::IndexOrComposeWithList(
                            IndexOrComposeWithListExprDisambiguation::Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: Int {
                                        element_ty: Symbol(
                                            EtherealTermSymbol(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                        ),
                        Ok(
                            FluffyTerm::Symbol(
                                EtherealTermSymbol {
                                    ty: EtherealTerm(`Type`),
                                    index: EtherealTermSymbolIndex(
                                        Type {
                                            attrs: EtherealTemplateSymbolAttrs {
                                                phantom: false,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                },
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
                    ExprTypeError::Derived(
                        DerivedExprTypeError::UnableToInferIndexExprType,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    1,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
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
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    3,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    0,
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm::Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::num::usize`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    0,
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::IndexOrComposeWithList(
                            IndexOrComposeWithListExprDisambiguation::Index(
                                FluffyDynamicDispatch {
                                    indirections: [],
                                    signature: Int {
                                        element_ty: Symbol(
                                            EtherealTermSymbol(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                        ),
                        Ok(
                            FluffyTerm::Symbol(
                                EtherealTermSymbol {
                                    ty: EtherealTerm(`Type`),
                                    index: EtherealTermSymbolIndex(
                                        Type {
                                            attrs: EtherealTemplateSymbolAttrs {
                                                phantom: false,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    25,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Derived(
                        DerivedExprTypeError::UnableToInferIndexExprType,
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    4,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    0,
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
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Original(
                        OriginalExprTypeError::NoMethodForType {
                            self_expr_ty: FluffyTerm::Solid(
                                SolidTerm(
                                    0,
                                ),
                            ),
                            ident_token: IdentToken {
                                ident: `swap`,
                                token_idx: TokenIdx(
                                    201,
                                ),
                            },
                        },
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Solid(
                                SolidTerm(
                                    0,
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
                disambiguation_and_ty_result: Err(
                    ExprTypeError::Original(
                        OriginalExprTypeError::NoMethodForType {
                            self_expr_ty: FluffyTerm::Solid(
                                SolidTerm(
                                    0,
                                ),
                            ),
                            ident_token: IdentToken {
                                ident: `swap`,
                                token_idx: TokenIdx(
                                    213,
                                ),
                            },
                        },
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::BlockTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [
            Ok(
                FluffyTerm::EntityPath(
                    TermEntityPath::TypeOntology(
                        TypePath(`core::num::usize`, `Extern`),
                    ),
                ),
            ),
            Ok(
                FluffyTerm::EntityPath(
                    TermEntityPath::TypeOntology(
                        TypePath(`core::num::usize`, `Extern`),
                    ),
                ),
            ),
            Ok(
                FluffyTerm::EntityPath(
                    TermEntityPath::TypeOntology(
                        TypePath(`core::num::usize`, `Extern`),
                    ),
                ),
            ),
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    Some(
                        SymbolType(
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
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
                ],
            },
            current_symbol_map: ArenaMap {
                data: [
                    None,
                    None,
                    None,
                ],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                    ),
                    None,
                    None,
                    None,
                ],
            },
            current_symbol_map: ArenaMap {
                data: [
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
                            path: TypePath(`core::slice::Slice`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Slice,
                            ),
                            arguments: [
                                FluffyTerm::Symbol(
                                    EtherealTermSymbol {
                                        ty: EtherealTerm(`Type`),
                                        index: EtherealTermSymbolIndex(
                                            Type {
                                                attrs: EtherealTemplateSymbolAttrs {
                                                    phantom: false,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                    },
                                ),
                            ],
                            base_ty_term: Some(
                                EtherealTerm(`Slice t`),
                            ),
                            place: MutableStackOwned {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        1,
                                    ),
                                ),
                            },
                        },
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::num::isize`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Num(
                                    Int(
                                        ISize,
                                    ),
                                ),
                            ),
                            arguments: [],
                            base_ty_term: Some(
                                EtherealTerm(`isize`),
                            ),
                            place: StackPure {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        2,
                                    ),
                                ),
                            },
                        },
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::num::isize`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Num(
                                    Int(
                                        ISize,
                                    ),
                                ),
                            ),
                            arguments: [],
                            base_ty_term: Some(
                                EtherealTerm(`isize`),
                            ),
                            place: StackPure {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        3,
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
                                    9,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    21,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    24,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    27,
                                ),
                                hole_kind: UnspecifiedIntegerType,
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
                                hole_kind: UnspecifiedIntegerType,
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        0,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ExplicitlyConvertible(
                                ExpectCasting {
                                    destination: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
                                        2,
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
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::usize`, `Extern`),
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
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
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
                                            TypePath(`core::num::isize`, `Extern`),
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
                                        TypePath(`core::num::isize`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
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
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::isize`, `Extern`),
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
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
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
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                            },
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
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 9,
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
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
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
                                idx: 10,
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        0,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 13,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
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
                                    expr_idx: 14,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::usize`, `Extern`),
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
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 17,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Symbol(
                                    EtherealTermSymbol {
                                        ty: EtherealTerm(`Type`),
                                        index: EtherealTermSymbolIndex(
                                            Type {
                                                attrs: EtherealTemplateSymbolAttrs {
                                                    phantom: false,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
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
                                expectee: FluffyTerm::Solid(
                                    SolidTerm(
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
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 19,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                            },
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
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 21,
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
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 22,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                            },
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
                                idx: 18,
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
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 19,
                                src: ExpectationSource {
                                    expr_idx: 25,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                            },
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
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 27,
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
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 36,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
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
                                idx: 22,
                                src: ExpectationSource {
                                    expr_idx: 28,
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        0,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 23,
                                src: ExpectationSource {
                                    expr_idx: 30,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsSort(
                                            TermUniverse(
                                                1,
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
                                idx: 24,
                                src: ExpectationSource {
                                    expr_idx: 31,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::num::usize`, `Extern`),
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
                                idx: 25,
                                src: ExpectationSource {
                                    expr_idx: 34,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Symbol(
                                    EtherealTermSymbol {
                                        ty: EtherealTerm(`Type`),
                                        index: EtherealTermSymbolIndex(
                                            Type {
                                                attrs: EtherealTemplateSymbolAttrs {
                                                    phantom: false,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
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
                                    expr_idx: 32,
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
                                idx: 27,
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
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
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
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 28,
                                src: ExpectationSource {
                                    expr_idx: 38,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                            },
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
                                idx: 29,
                                src: ExpectationSource {
                                    expr_idx: 40,
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
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 30,
                                src: ExpectationSource {
                                    expr_idx: 41,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
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
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 31,
                                src: ExpectationSource {
                                    expr_idx: 44,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
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
                                idx: 32,
                                src: ExpectationSource {
                                    expr_idx: 45,
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
                                idx: 33,
                                src: ExpectationSource {
                                    expr_idx: 53,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: Some(
            EtherealTerm(`isize`),
        ),
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
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
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    2,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    3,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    4,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    4,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    6,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    7,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    8,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    9,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    10,
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
                        ExprDisambiguation::List(
                            ListExprDisambiguation::NewList,
                        ),
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    11,
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Implicit,
                                    variance: Independent,
                                    parameter_variable: Some(
                                        EtherealTermVariable {
                                            ty: EtherealTerm(`Type`),
                                            idx: 0,
                                        },
                                    ),
                                    parameter_ty: EtherealTerm(`Type`),
                                    return_ty: EtherealTerm(`fn(move  Slice variable_ad_hoc_fmt) -> unit`),
                                },
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    16,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    16,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    17,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    18,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    19,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    20,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    21,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    22,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    23,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    24,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    25,
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::List(
                            ListExprDisambiguation::NewList,
                        ),
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    26,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    27,
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
                ],
            },
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
                                    11,
                                ),
                                hole_kind: Any,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    0,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    1,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    2,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    3,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    5,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    6,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    7,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    8,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    9,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    10,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 7,
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
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    12,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 32,
                                    },
                                ),
                                refined_path: Left(
                                    Slice,
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            12,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: Hollow(
                                                HollowTerm(
                                                    13,
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 4,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    27,
                                ),
                                hole_kind: Any,
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
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    17,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    18,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    19,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    20,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    21,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    22,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
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
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    24,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    25,
                                ),
                                hole_kind: UnspecifiedIntegerType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                refined_path: Left(
                                    List,
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            15,
                                        ),
                                    ),
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        1,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        1,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
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
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        2,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        2,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
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
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        3,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        3,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
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
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
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
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
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
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        4,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        4,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
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
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        5,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        5,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
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
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        6,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        6,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
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
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        7,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        7,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        8,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        8,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        9,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        9,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        10,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        10,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
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
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        11,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Implicit,
                                        variance: Independent,
                                        parameter_variable: Some(
                                            EtherealTermVariable {
                                                ty: EtherealTerm(`Type`),
                                                idx: 0,
                                            },
                                        ),
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`fn(move  Slice variable_ad_hoc_fmt) -> unit`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: FluffyTerm::Variable(
                                                            EtherealTermVariable {
                                                                ty: EtherealTerm(`Type`),
                                                                idx: 0,
                                                            },
                                                        ),
                                                        substitute: FluffyTerm::Hollow(
                                                            HollowTerm(
                                                                12,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: Hollow(
                                                                    HollowTerm(
                                                                        13,
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
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 14,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
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
                                idx: 14,
                                src: ExpectationSource {
                                    expr_idx: 15,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        16,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            15,
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
                                        16,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        16,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            15,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 17,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        17,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        17,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            15,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 18,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        18,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        18,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            15,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 19,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        19,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        19,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            15,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 19,
                                src: ExpectationSource {
                                    expr_idx: 20,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        20,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        20,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            15,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 21,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        21,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        21,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            15,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 22,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        22,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        22,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            15,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 22,
                                src: ExpectationSource {
                                    expr_idx: 23,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        23,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        23,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            15,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 23,
                                src: ExpectationSource {
                                    expr_idx: 24,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        24,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        24,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            15,
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
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        25,
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        25,
                                                    ),
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 25,
                                src: ExpectationSource {
                                    expr_idx: 27,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Hollow(
                                    HollowTerm(
                                        26,
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
                                idx: 26,
                                src: ExpectationSource {
                                    expr_idx: 28,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                            },
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
                                idx: 27,
                                src: ExpectationSource {
                                    expr_idx: 29,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
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
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
            ],
        },
        expr_ty_infos: [
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`Ref 'static`),
                                    argument: EtherealTerm(`str`),
                                    shift: 0,
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`Ref 'static`),
                                    argument: EtherealTerm(`str`),
                                    shift: 0,
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`Ref 'static`),
                                    argument: EtherealTerm(`str`),
                                    shift: 0,
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`Ref 'static`),
                                    argument: EtherealTerm(`str`),
                                    shift: 0,
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`Ref 'static`),
                                    argument: EtherealTerm(`str`),
                                    shift: 0,
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`Ref 'static`),
                                    argument: EtherealTerm(`str`),
                                    shift: 0,
                                },
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
                        ExprDisambiguation::List(
                            ListExprDisambiguation::NewList,
                        ),
                        Ok(
                            FluffyTerm::Hollow(
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
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Implicit,
                                    variance: Independent,
                                    parameter_variable: Some(
                                        EtherealTermVariable {
                                            ty: EtherealTerm(`Type`),
                                            idx: 0,
                                        },
                                    ),
                                    parameter_ty: EtherealTerm(`Type`),
                                    return_ty: EtherealTerm(`fn(move  Slice variable_ad_hoc_fmt) -> unit`),
                                },
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                            ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                        ),
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`Ref 'static`),
                                    argument: EtherealTerm(`str`),
                                    shift: 0,
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`Ref 'static`),
                                    argument: EtherealTerm(`str`),
                                    shift: 0,
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`Ref 'static`),
                                    argument: EtherealTerm(`str`),
                                    shift: 0,
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`Ref 'static`),
                                    argument: EtherealTerm(`str`),
                                    shift: 0,
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`Ref 'static`),
                                    argument: EtherealTerm(`str`),
                                    shift: 0,
                                },
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Application(
                                EtherealTermApplication {
                                    function: EtherealTerm(`Ref 'static`),
                                    argument: EtherealTerm(`str`),
                                    shift: 0,
                                },
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
                        ExprDisambiguation::Trivial,
                        Err(
                            ExprTypeError::Derived(
                                DerivedExprTypeError::CurrentSymbolTypeError,
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: None,
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::List(
                            ListExprDisambiguation::NewList,
                        ),
                        Ok(
                            FluffyTerm::Hollow(
                                HollowTerm(
                                    6,
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::bool`, `Extern`),
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
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::EntityPath(
                                TermEntityPath::TypeOntology(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                        ),
                    ),
                ),
                expectation_rule_idx: Some(
                    17,
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
                ],
            },
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
                                    6,
                                ),
                                hole_kind: Any,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 7,
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
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    7,
                                ),
                                hole_kind: ImplicitType,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 32,
                                    },
                                ),
                                refined_path: Left(
                                    Slice,
                                ),
                                arguments: [
                                    Hollow(
                                        HollowTerm(
                                            2,
                                        ),
                                    ),
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Ritchie {
                                ritchie_kind: FnType,
                                params: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: Hollow(
                                                HollowTerm(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            Id {
                                                value: 4,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expr(
                                    17,
                                ),
                                hole_kind: Any,
                                fill: None,
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::Unresolved,
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                refined_path: Left(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Ref 'static`),
                                        argument: EtherealTerm(`str`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Application(
                                                    EtherealTermApplication {
                                                        function: EtherealTerm(`Ref 'static`),
                                                        argument: EtherealTerm(`str`),
                                                        shift: 0,
                                                    },
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
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
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Ref 'static`),
                                        argument: EtherealTerm(`str`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Application(
                                                    EtherealTermApplication {
                                                        function: EtherealTerm(`Ref 'static`),
                                                        argument: EtherealTerm(`str`),
                                                        shift: 0,
                                                    },
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
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
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Ref 'static`),
                                        argument: EtherealTerm(`str`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Application(
                                                    EtherealTermApplication {
                                                        function: EtherealTerm(`Ref 'static`),
                                                        argument: EtherealTerm(`str`),
                                                        shift: 0,
                                                    },
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
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
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Ref 'static`),
                                        argument: EtherealTerm(`str`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Application(
                                                    EtherealTermApplication {
                                                        function: EtherealTerm(`Ref 'static`),
                                                        argument: EtherealTerm(`str`),
                                                        shift: 0,
                                                    },
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
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
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Ref 'static`),
                                        argument: EtherealTerm(`str`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Application(
                                                    EtherealTermApplication {
                                                        function: EtherealTerm(`Ref 'static`),
                                                        argument: EtherealTerm(`str`),
                                                        shift: 0,
                                                    },
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            0,
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
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Ref 'static`),
                                        argument: EtherealTerm(`str`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Application(
                                                    EtherealTermApplication {
                                                        function: EtherealTerm(`Ref 'static`),
                                                        argument: EtherealTerm(`str`),
                                                        shift: 0,
                                                    },
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
                                                    HollowTerm(
                                                        0,
                                                    ),
                                                ),
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
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
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
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Implicit,
                                        variance: Independent,
                                        parameter_variable: Some(
                                            EtherealTermVariable {
                                                ty: EtherealTerm(`Type`),
                                                idx: 0,
                                            },
                                        ),
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`fn(move  Slice variable_ad_hoc_fmt) -> unit`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [
                                                    ImplicitParameterSubstitution {
                                                        variable: FluffyTerm::Variable(
                                                            EtherealTermVariable {
                                                                ty: EtherealTerm(`Type`),
                                                                idx: 0,
                                                            },
                                                        ),
                                                        substitute: FluffyTerm::Hollow(
                                                            HollowTerm(
                                                                2,
                                                            ),
                                                        ),
                                                    },
                                                ],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                                                    ritchie_kind: FnType,
                                                    parameter_contracted_tys: [
                                                        FluffyTermRitchieParameter::Regular(
                                                            FluffyTermRitchieRegularParameter {
                                                                contract: Move,
                                                                ty: Hollow(
                                                                    HollowTerm(
                                                                        3,
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
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::unit`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            5,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Ref 'static`),
                                        argument: EtherealTerm(`str`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Application(
                                                    EtherealTermApplication {
                                                        function: EtherealTerm(`Ref 'static`),
                                                        argument: EtherealTerm(`str`),
                                                        shift: 0,
                                                    },
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            5,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Ref 'static`),
                                        argument: EtherealTerm(`str`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Application(
                                                    EtherealTermApplication {
                                                        function: EtherealTerm(`Ref 'static`),
                                                        argument: EtherealTerm(`str`),
                                                        shift: 0,
                                                    },
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            5,
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
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Ref 'static`),
                                        argument: EtherealTerm(`str`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Application(
                                                    EtherealTermApplication {
                                                        function: EtherealTerm(`Ref 'static`),
                                                        argument: EtherealTerm(`str`),
                                                        shift: 0,
                                                    },
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            5,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 13,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Ref 'static`),
                                        argument: EtherealTerm(`str`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Application(
                                                    EtherealTermApplication {
                                                        function: EtherealTerm(`Ref 'static`),
                                                        argument: EtherealTerm(`str`),
                                                        shift: 0,
                                                    },
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            5,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 14,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Ref 'static`),
                                        argument: EtherealTerm(`str`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Application(
                                                    EtherealTermApplication {
                                                        function: EtherealTerm(`Ref 'static`),
                                                        argument: EtherealTerm(`str`),
                                                        shift: 0,
                                                    },
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::Hollow(
                                        HollowTerm(
                                            5,
                                        ),
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 14,
                                src: ExpectationSource {
                                    expr_idx: 15,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Application(
                                    EtherealTermApplication {
                                        function: EtherealTerm(`Ref 'static`),
                                        argument: EtherealTerm(`str`),
                                        shift: 0,
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::Application(
                                                    EtherealTermApplication {
                                                        function: EtherealTerm(`Ref 'static`),
                                                        argument: EtherealTerm(`str`),
                                                        shift: 0,
                                                    },
                                                ),
                                                contract: Move,
                                                expected: FluffyTerm::Hollow(
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
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 17,
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
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 18,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::bool`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Err(
                                        FluffyTermExpectationError::Original(
                                            OriginalFluffyTermExpectationError::ExpectedCoersion {
                                                expectee: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                                contract: None,
                                                expected: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::bool`, `Extern`),
                                                    ),
                                                ),
                                            },
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
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 19,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::unit`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
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