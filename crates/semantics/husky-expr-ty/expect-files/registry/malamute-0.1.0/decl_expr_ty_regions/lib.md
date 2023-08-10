[
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`malamute::OneVsAll`, `Enum`),
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
                        ExprDisambiguation::Trivial,
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
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 3,
                                },
                            ),
                        ),
                    ),
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 9,
                                },
                            ),
                        ),
                    ),
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [
                        SolidTermData::SymbolAtPlace {
                            term: EtherealTermSymbol {
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
                            place: Const,
                        },
                    ],
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                        ExprDisambiguation::Trivial,
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
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 3,
                                },
                            ),
                        ),
                    ),
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 9,
                                },
                            ),
                        ),
                    ),
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [
                        SolidTermData::SymbolAtPlace {
                            term: EtherealTermSymbol {
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
                            place: Const,
                        },
                    ],
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`malamute::narrow_down`, `Gn`),
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
                                    TypePath(`core::num::i32`, `Extern`),
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
                        ExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Explicit,
                                    variance: Independent,
                                    parameter_variable: Some(
                                        EtherealTermVariable {
                                            ty: EtherealTerm(`Type`),
                                            idx: 0,
                                        },
                                    ),
                                    parameter_ty: EtherealTerm(`Type`),
                                    return_ty: EtherealTerm(`independent variable_ad_hoc_fmt -> Type`),
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
                    5,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Explicit,
                                    variance: Independent,
                                    parameter_variable: None,
                                    parameter_ty: EtherealTerm(`t`),
                                    return_ty: EtherealTerm(`Type`),
                                },
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
                            FluffyTerm::Solid(
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
                        ExprDisambiguation::Trivial,
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
                    8,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [
            Ok(
                FluffyTerm::EntityPath(
                    TermEntityPath::TypeOntology(
                        TypePath(`core::num::i32`, `Extern`),
                    ),
                ),
            ),
            Ok(
                FluffyTerm::Literal(
                    TermLiteral::I32(
                        5,
                    ),
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
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
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
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 3,
                                },
                            ),
                        ),
                    ),
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 9,
                                },
                            ),
                        ),
                    ),
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 10,
                                },
                            ),
                        ),
                    ),
                    None,
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [
                        SolidTermData::SymbolAtPlace {
                            term: EtherealTermSymbol {
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
                            place: Const,
                        },
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::num::f32`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Num(
                                    Float(
                                        F32,
                                    ),
                                ),
                            ),
                            arguments: [],
                            base_ty_term: Some(
                                EtherealTerm(`f32`),
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
                            path: TypePath(`core::num::i32`, `Extern`),
                            refined_path: Left(
                                PreludeTypePath::Num(
                                    Int(
                                        I32,
                                    ),
                                ),
                            ),
                            arguments: [],
                            base_ty_term: Some(
                                EtherealTerm(`i32`),
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
                    entries: [],
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
                                        1,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm::EntityPath(
                                        TermEntityPath::TypeOntology(
                                            TypePath(`core::num::i32`, `Extern`),
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
                                        TypePath(`core::num::i32`, `Extern`),
                                    ),
                                ),
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
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::Sort,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: Some(
                                            EtherealTermVariable {
                                                ty: EtherealTerm(`Type`),
                                                idx: 0,
                                            },
                                        ),
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`independent variable_ad_hoc_fmt -> Type`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::Curry(
                                                    EtherealTermCurry {
                                                        curry_kind: Explicit,
                                                        variance: Independent,
                                                        parameter_variable: None,
                                                        parameter_ty: EtherealTerm(`variable_ad_hoc_fmt`),
                                                        return_ty: EtherealTerm(`Type`),
                                                    },
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Independent,
                                                    parameter_symbol: Some(
                                                        FluffyTerm::Variable(
                                                            EtherealTermVariable {
                                                                ty: EtherealTerm(`Type`),
                                                                idx: 0,
                                                            },
                                                        ),
                                                    ),
                                                    parameter_ty: FluffyTerm::Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    return_ty: FluffyTerm::Curry(
                                                        EtherealTermCurry {
                                                            curry_kind: Explicit,
                                                            variance: Independent,
                                                            parameter_variable: None,
                                                            parameter_ty: EtherealTerm(`variable_ad_hoc_fmt`),
                                                            return_ty: EtherealTerm(`Type`),
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::Sort,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`t`),
                                        return_ty: EtherealTerm(`Type`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::Category(
                                                    TermCategory {
                                                        universe: TermUniverse(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Independent,
                                                    parameter_symbol: None,
                                                    parameter_ty: FluffyTerm::Symbol(
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
                                                    return_ty: FluffyTerm::Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 3,
                                            },
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
                                        1,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 8,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `malamute`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
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
                        ExprDisambiguation::Trivial,
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
                        ExprDisambiguation::TypePath(
                            InstanceConstructor,
                        ),
                        Ok(
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Explicit,
                                    variance: Invariant,
                                    parameter_variable: None,
                                    parameter_ty: EtherealTerm(`Type`),
                                    return_ty: EtherealTerm(`Trait`),
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
                        ExprDisambiguation::TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Explicit,
                                    variance: Independent,
                                    parameter_variable: Some(
                                        EtherealTermVariable {
                                            ty: EtherealTerm(`Type`),
                                            idx: 0,
                                        },
                                    ),
                                    parameter_ty: EtherealTerm(`Type`),
                                    return_ty: EtherealTerm(`independent variable_ad_hoc_fmt -> Type`),
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
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Explicit,
                                    variance: Independent,
                                    parameter_variable: None,
                                    parameter_ty: EtherealTerm(`Type`),
                                    return_ty: EtherealTerm(`independent variable_ad_hoc_fmt -> Trait`),
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
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Explicit,
                                    variance: Independent,
                                    parameter_variable: None,
                                    parameter_ty: EtherealTerm(`variable_ad_hoc_fmt`),
                                    return_ty: EtherealTerm(`Trait`),
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
                                    TypePath(`core::basic::Trait`, `Extern`),
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
                            OntologyConstructor,
                        ),
                        Ok(
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Explicit,
                                    variance: Independent,
                                    parameter_variable: Some(
                                        EtherealTermVariable {
                                            ty: EtherealTerm(`Type`),
                                            idx: 0,
                                        },
                                    ),
                                    parameter_ty: EtherealTerm(`Type`),
                                    return_ty: EtherealTerm(`independent variable_ad_hoc_fmt -> Type`),
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
                    9,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        ExprDisambiguation::Trivial,
                        Ok(
                            FluffyTerm::Curry(
                                EtherealTermCurry {
                                    curry_kind: Explicit,
                                    variance: Independent,
                                    parameter_variable: None,
                                    parameter_ty: EtherealTerm(`t`),
                                    return_ty: EtherealTerm(`Type`),
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
                            FluffyTerm::Solid(
                                SolidTerm(
                                    0,
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
                    12,
                ),
            },
        ],
        extra_expr_errors: [],
        expr_fluffy_terms: [
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
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
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
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [],
            },
            current_symbol_map: ArenaMap {
                data: [
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 3,
                                },
                            ),
                        ),
                    ),
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 9,
                                },
                            ),
                        ),
                    ),
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [
                        SolidTermData::SymbolAtPlace {
                            term: EtherealTermSymbol {
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
                            place: Const,
                        },
                    ],
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
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
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Invariant,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`Trait`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::Trait`, `Extern`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Invariant,
                                                    parameter_symbol: None,
                                                    parameter_ty: FluffyTerm::Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    return_ty: FluffyTerm::EntityPath(
                                                        TermEntityPath::TypeOntology(
                                                            TypePath(`core::basic::Trait`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
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
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: Some(
                                            EtherealTermVariable {
                                                ty: EtherealTerm(`Type`),
                                                idx: 0,
                                            },
                                        ),
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`independent variable_ad_hoc_fmt -> Type`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
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
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`independent variable_ad_hoc_fmt -> Trait`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::Curry(
                                                    EtherealTermCurry {
                                                        curry_kind: Explicit,
                                                        variance: Independent,
                                                        parameter_variable: None,
                                                        parameter_ty: EtherealTerm(`variable_ad_hoc_fmt`),
                                                        return_ty: EtherealTerm(`Trait`),
                                                    },
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Independent,
                                                    parameter_symbol: None,
                                                    parameter_ty: FluffyTerm::Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    return_ty: FluffyTerm::Curry(
                                                        EtherealTermCurry {
                                                            curry_kind: Explicit,
                                                            variance: Independent,
                                                            parameter_variable: None,
                                                            parameter_ty: EtherealTerm(`variable_ad_hoc_fmt`),
                                                            return_ty: EtherealTerm(`Trait`),
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
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
                                expectee: FluffyTerm::Category(
                                    TermCategory {
                                        universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`variable_ad_hoc_fmt`),
                                        return_ty: EtherealTerm(`Trait`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::Trait`, `Extern`),
                                                    ),
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Independent,
                                                    parameter_symbol: None,
                                                    parameter_ty: FluffyTerm::Variable(
                                                        EtherealTermVariable {
                                                            ty: EtherealTerm(`Type`),
                                                            idx: 0,
                                                        },
                                                    ),
                                                    return_ty: FluffyTerm::EntityPath(
                                                        TermEntityPath::TypeOntology(
                                                            TypePath(`core::basic::Trait`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: Variable(
                                        EtherealTermVariable(
                                            Id {
                                                value: 1,
                                            },
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
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::EntityPath(
                                    TermEntityPath::TypeOntology(
                                        TypePath(`core::basic::Trait`, `Extern`),
                                    ),
                                ),
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::Sort,
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
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: Some(
                                            EtherealTermVariable {
                                                ty: EtherealTerm(`Type`),
                                                idx: 0,
                                            },
                                        ),
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`independent variable_ad_hoc_fmt -> Type`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::Curry(
                                                    EtherealTermCurry {
                                                        curry_kind: Explicit,
                                                        variance: Independent,
                                                        parameter_variable: None,
                                                        parameter_ty: EtherealTerm(`variable_ad_hoc_fmt`),
                                                        return_ty: EtherealTerm(`Type`),
                                                    },
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Independent,
                                                    parameter_symbol: Some(
                                                        FluffyTerm::Variable(
                                                            EtherealTermVariable {
                                                                ty: EtherealTerm(`Type`),
                                                                idx: 0,
                                                            },
                                                        ),
                                                    ),
                                                    parameter_ty: FluffyTerm::Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                    return_ty: FluffyTerm::Curry(
                                                        EtherealTermCurry {
                                                            curry_kind: Explicit,
                                                            variance: Independent,
                                                            parameter_variable: None,
                                                            parameter_ty: EtherealTerm(`variable_ad_hoc_fmt`),
                                                            return_ty: EtherealTerm(`Type`),
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 9,
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
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::Sort,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`t`),
                                        return_ty: EtherealTerm(`Type`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                template_parameter_substitutions: [],
                                                return_ty: FluffyTerm::Category(
                                                    TermCategory {
                                                        universe: TermUniverse(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Independent,
                                                    parameter_symbol: None,
                                                    parameter_ty: FluffyTerm::Symbol(
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
                                                    return_ty: FluffyTerm::Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        ExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 3,
                                            },
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
                                        1,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 12,
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `malamute`,
                                    trai_path: TraitPath(`core::ops::Unveil`),
                                    ty_sketch: TypeSketch::Path(
                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                    ),
                                    disambiguator: 0,
                                },
                                ident: `Output`,
                                item_kind: AssociatedType,
                            },
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
                    0,
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
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: ArenaMap {
                data: [
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 3,
                                },
                            ),
                        ),
                    ),
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 9,
                                },
                            ),
                        ),
                    ),
                ],
            },
            current_symbol_map: ArenaMap {
                data: [],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [
                        SolidTermData::SymbolAtPlace {
                            term: EtherealTermSymbol {
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
                            place: Const,
                        },
                    ],
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
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
                                            OriginalFluffyTermExpectationError::ExpectedCategory {
                                                expectee: FluffyTerm::EntityPath(
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
                    ],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: Some(
            EtherealTerm(`OneVsAll t a`),
        ),
    },
]