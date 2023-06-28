[
    ExprTypeRegion {
        path: RegionPath::Decl(
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
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
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 2,
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
                    2,
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
                                    value: 2,
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
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: MutableStackOwned {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            1,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 67,
                                    },
                                ),
                                refined_path: Left(
                                    Slice,
                                ),
                                arguments: [
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                ),
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
                            src: ExpectationSource {
                                expr_idx: 1,
                                kind: Expr,
                            },
                            expectee: Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                            data: EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: Sort,
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    EqsFunctionCallType(
                                        ExpectEqsFunctionTypeOutcome {
                                            implicit_parameter_substitutions: [],
                                            return_ty: Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                            variant: Curry {
                                                variance: Covariant,
                                                parameter_symbol: None,
                                                parameter_ty: Category(
                                                    TermCategory {
                                                        universe: TermUniverse(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                                return_ty: Category(
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
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 2,
                                kind: Expr,
                            },
                            expectee: Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            data: CurryDestination(
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
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 3,
                                kind: Expr,
                            },
                            expectee: Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            data: EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        TermUniverse(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
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
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
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
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 2,
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
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            Ontology,
                        ),
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
                        TypePath(
                            Ontology,
                        ),
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
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: MutableStackOwned {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            1,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 67,
                                    },
                                ),
                                refined_path: Left(
                                    Slice,
                                ),
                                arguments: [
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            2,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 54,
                                    },
                                ),
                                refined_path: Left(
                                    Num(
                                        Int(
                                            ISize,
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 54,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            3,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 54,
                                    },
                                ),
                                refined_path: Left(
                                    Num(
                                        Int(
                                            ISize,
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 54,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
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
                            src: ExpectationSource {
                                expr_idx: 1,
                                kind: Expr,
                            },
                            expectee: Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                            data: EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: Sort,
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    EqsFunctionCallType(
                                        ExpectEqsFunctionTypeOutcome {
                                            implicit_parameter_substitutions: [],
                                            return_ty: Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                            variant: Curry {
                                                variance: Covariant,
                                                parameter_symbol: None,
                                                parameter_ty: Category(
                                                    TermCategory {
                                                        universe: TermUniverse(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                                return_ty: Category(
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
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 2,
                                kind: Expr,
                            },
                            expectee: Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            data: CurryDestination(
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
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 3,
                                kind: Expr,
                            },
                            expectee: Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            data: EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        TermUniverse(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 4,
                                kind: Expr,
                            },
                            expectee: Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            data: EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        TermUniverse(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 5,
                                kind: Expr,
                            },
                            expectee: Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            data: EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        TermUniverse(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
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
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::partition`, `Fn`),
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
                            Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 2,
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
                    2,
                ),
            },
            ExprTypeInfo {
                disambiguation_and_ty_result: Ok(
                    (
                        TypePath(
                            Ontology,
                        ),
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
                        TypePath(
                            Ontology,
                        ),
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
                            Ontology,
                        ),
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
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                solid_terms: SolidTerms {
                    entries: VecSet {
                        data: [
                            PlaceTypeOntology {
                                place: MutableStackOwned {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            1,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 67,
                                    },
                                ),
                                refined_path: Left(
                                    Slice,
                                ),
                                arguments: [
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            2,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 54,
                                    },
                                ),
                                refined_path: Left(
                                    Num(
                                        Int(
                                            ISize,
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 54,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            PlaceTypeOntology {
                                place: StackPure {
                                    location: StackLocationIdx(
                                        LocalSymbolIdx(
                                            3,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 54,
                                    },
                                ),
                                refined_path: Left(
                                    Num(
                                        Int(
                                            ISize,
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 54,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
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
                            src: ExpectationSource {
                                expr_idx: 1,
                                kind: Expr,
                            },
                            expectee: Curry(
                                EtherealTermCurry(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                            data: EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: Sort,
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    EqsFunctionCallType(
                                        ExpectEqsFunctionTypeOutcome {
                                            implicit_parameter_substitutions: [],
                                            return_ty: Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
                                            variant: Curry {
                                                variance: Covariant,
                                                parameter_symbol: None,
                                                parameter_ty: Category(
                                                    TermCategory {
                                                        universe: TermUniverse(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                                return_ty: Category(
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
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 2,
                                kind: Expr,
                            },
                            expectee: Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            data: CurryDestination(
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
                            resolve_progress: Unresolved,
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 3,
                                kind: Expr,
                            },
                            expectee: Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            data: EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        TermUniverse(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 4,
                                kind: Expr,
                            },
                            expectee: Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            data: EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        TermUniverse(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 5,
                                kind: Expr,
                            },
                            expectee: Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            data: EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        TermUniverse(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 6,
                                kind: Expr,
                            },
                            expectee: Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            data: EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    EqsSort(
                                        TermUniverse(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
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
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
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
            EntityNodePath::ModuleItem(
                ModuleItemNodePath::Fugitive(
                    FugitiveNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
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
                    data: [],
                },
                first_unresolved_expectation: 0,
            },
        },
        return_ty: None,
        self_ty: None,
    },
]