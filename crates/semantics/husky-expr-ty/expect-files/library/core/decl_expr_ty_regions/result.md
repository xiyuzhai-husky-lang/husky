[
    ExprTypeRegion {
        path: RegionPath::Decl(
            EntitySynNodePath::ModuleItem(
                ModuleItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::result::Result`, `Enum`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        expr_ty_infos: [],
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
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
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
                                    value: 2,
                                },
                            ),
                        ),
                    ),
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 3,
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
                    entries: [],
                },
                hollow_terms: HollowTerms {
                    entries: [],
                    first_unresolved_term_idx: 0,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            EntitySynNodePath::ImplBlock(
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `core::result`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: Path(
                                TypePath(
                                    Id {
                                        value: 31,
                                    },
                                ),
                            ),
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
                                        value: 1,
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
                            OntologyConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 9,
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
                        Trivial,
                        Ok(
                            Curry(
                                EtherealTermCurry(
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
                        Trivial,
                        Ok(
                            Category(
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
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Curry(
                                EtherealTermCurry(
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
                        Trivial,
                        Ok(
                            Category(
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
                        Trivial,
                        Ok(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 65,
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
                            OntologyConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 9,
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
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Category(
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
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 8,
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
                        Trivial,
                        Ok(
                            Category(
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
                    10,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Category(
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
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
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
                                    value: 2,
                                },
                            ),
                        ),
                    ),
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
                                    value: 5,
                                },
                            ),
                        ),
                    ),
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 6,
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
                                    final_destination: FinalDestination::AnyOriginal,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
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
                                                implicit_parameter_substitutions: [],
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
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`independent Type -> Type`),
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
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`independent Type -> Trait`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                implicit_parameter_substitutions: [],
                                                return_ty: FluffyTerm::Curry(
                                                    EtherealTermCurry {
                                                        curry_kind: Explicit,
                                                        variance: Independent,
                                                        parameter_variable: None,
                                                        parameter_ty: EtherealTerm(`Type`),
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
                                                            parameter_ty: EtherealTerm(`Type`),
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
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
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
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`Trait`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                implicit_parameter_substitutions: [],
                                                return_ty: FluffyTerm::EntityPath(
                                                    TermEntityPath::TypeOntology(
                                                        TypePath(`core::basic::Trait`, `Extern`),
                                                    ),
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
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
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
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`independent Type -> Type`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                implicit_parameter_substitutions: [],
                                                return_ty: FluffyTerm::Curry(
                                                    EtherealTermCurry {
                                                        curry_kind: Explicit,
                                                        variance: Independent,
                                                        parameter_variable: None,
                                                        parameter_ty: EtherealTerm(`Type`),
                                                        return_ty: EtherealTerm(`Type`),
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
                                                            parameter_ty: EtherealTerm(`Type`),
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
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`Type`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                implicit_parameter_substitutions: [],
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
                                                    parameter_ty: FluffyTerm::Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
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
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 10,
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 11,
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
            EntitySynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `core::result`,
                                    trai_path: TraitPath(`core::ops::Unveil`),
                                    ty_sketch: Path(
                                        TypePath(
                                            Id {
                                                value: 31,
                                            },
                                        ),
                                    ),
                                    disambiguator: 0,
                                },
                                ident: `Continue`,
                                item_kind: AssociatedType,
                            },
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
                            Category(
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
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
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
                                    value: 5,
                                },
                            ),
                        ),
                    ),
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 6,
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
        self_ty: Some(
            EtherealTerm(`Result t s`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            EntitySynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `core::result`,
                                    trai_path: TraitPath(`core::ops::Unveil`),
                                    ty_sketch: Path(
                                        TypePath(
                                            Id {
                                                value: 31,
                                            },
                                        ),
                                    ),
                                    disambiguator: 0,
                                },
                                ident: `branch`,
                                item_kind: MethodFn,
                            },
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
                            OntologyConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 9,
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
                            Category(
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
                        Trivial,
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 8,
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
                            Category(
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
                    3,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Category(
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
                        TypePath(
                            OntologyConstructor,
                        ),
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 9,
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
                        Trivial,
                        Ok(
                            Category(
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
                    6,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 8,
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
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Category(
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
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        Trivial,
                        Ok(
                            Category(
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
                            Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    ),
                ],
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
                                    value: 5,
                                },
                            ),
                        ),
                    ),
                    Some(
                        Symbol(
                            EtherealTermSymbol(
                                Id {
                                    value: 6,
                                },
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
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: [
                        SolidTermData::TypeOntologyAtPlace {
                            path: TypePath(`core::result::Result`, `Enum`),
                            refined_path: Right(
                                CustomTypePath(
                                    TypePath(`core::result::Result`, `Enum`),
                                ),
                            ),
                            arguments: [
                                FluffyTerm::Symbol(
                                    EtherealTermSymbol {
                                        ty: EtherealTerm(`Type`),
                                        idx: 1,
                                    },
                                ),
                                FluffyTerm::Symbol(
                                    EtherealTermSymbol {
                                        ty: EtherealTerm(`Type`),
                                        idx: 3,
                                    },
                                ),
                            ],
                            base_ty_term: Some(
                                EtherealTerm(`Result t s`),
                            ),
                            place: StackPure {
                                location: StackLocationIdx(
                                    LocalSymbolIdx(
                                        4,
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
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::Sort,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 0,
                                src: ExpectationSource {
                                    expr_idx: 0,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`independent Type -> Type`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                implicit_parameter_substitutions: [],
                                                return_ty: FluffyTerm::Curry(
                                                    EtherealTermCurry {
                                                        curry_kind: Explicit,
                                                        variance: Independent,
                                                        parameter_variable: None,
                                                        parameter_ty: EtherealTerm(`Type`),
                                                        return_ty: EtherealTerm(`Type`),
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
                                                            parameter_ty: EtherealTerm(`Type`),
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
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`Type`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                implicit_parameter_substitutions: [],
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
                                                    parameter_ty: FluffyTerm::Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
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
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
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
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
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
                                    final_destination: FinalDestination::Sort,
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
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`independent Type -> Type`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                implicit_parameter_substitutions: [],
                                                return_ty: FluffyTerm::Curry(
                                                    EtherealTermCurry {
                                                        curry_kind: Explicit,
                                                        variance: Independent,
                                                        parameter_variable: None,
                                                        parameter_ty: EtherealTerm(`Type`),
                                                        return_ty: EtherealTerm(`Type`),
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
                                                            parameter_ty: EtherealTerm(`Type`),
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
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
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
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm::Curry(
                                    EtherealTermCurry {
                                        curry_kind: Explicit,
                                        variance: Independent,
                                        parameter_variable: None,
                                        parameter_ty: EtherealTerm(`Type`),
                                        return_ty: EtherealTerm(`Type`),
                                    },
                                ),
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::EqsFunctionCallType(
                                            ExpectEqsFunctionTypeOutcome {
                                                implicit_parameter_substitutions: [],
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
                                                    parameter_ty: FluffyTerm::Category(
                                                        TermCategory {
                                                            universe: TermUniverse(
                                                                1,
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
        self_ty: Some(
            EtherealTerm(`Result t s`),
        ),
    },
]