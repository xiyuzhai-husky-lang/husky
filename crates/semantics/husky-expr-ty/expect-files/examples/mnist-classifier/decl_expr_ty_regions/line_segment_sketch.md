[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                    0,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        Tilde(
                            Leash,
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
                    1,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        List(
                            ListFunctor,
                        ),
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
                    2,
                ),
            },
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    TypeOntologyDeclError {
                                        path: TypePath(
                                            Id {
                                                value: 82,
                                            },
                                        ),
                                    },
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
                    entries: [],
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
                                expr_idx: 0,
                                kind: Expr,
                            },
                            expectee: Category(
                                TermCategory {
                                    universe: TermUniverse(
                                        1,
                                    ),
                                },
                            ),
                            data: EqsExactly(
                                ExpectSubtype {
                                    expected: Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                },
                            ),
                            resolve_progress: Resolved(
                                Ok(
                                    EqsExactly(
                                        ExpectSubtypeOutcome,
                                    ),
                                ),
                            ),
                        },
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 1,
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
                                expr_idx: 2,
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
                                    EqsRitchieCallType(
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
            DeclRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
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
                                        value: 78,
                                    },
                                ),
                                refined_path: Left(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 78,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 78,
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
                                            1,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 54,
                                    },
                                ),
                                refined_path: Right(
                                    Num(
                                        Float(
                                            F32,
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
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
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
            DeclRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
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
                                        value: 78,
                                    },
                                ),
                                refined_path: Left(
                                    CustomTypePath(
                                        TypePath(
                                            Id {
                                                value: 78,
                                            },
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 78,
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
                                            1,
                                        ),
                                    ),
                                },
                                path: TypePath(
                                    Id {
                                        value: 54,
                                    },
                                ),
                                refined_path: Right(
                                    Num(
                                        Float(
                                            F32,
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
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
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
            DeclRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                    None,
                    None,
                    None,
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
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
                                        value: 41,
                                    },
                                ),
                                refined_path: Right(
                                    Borrow(
                                        Leash,
                                    ),
                                ),
                                arguments: [
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 73,
                                                },
                                            ),
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 5,
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
                                        value: 44,
                                    },
                                ),
                                refined_path: Right(
                                    Num(
                                        Int(
                                            I32,
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
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
                                refined_path: Right(
                                    Num(
                                        Float(
                                            F32,
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
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
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
            DeclRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
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
                                        value: 41,
                                    },
                                ),
                                refined_path: Right(
                                    Borrow(
                                        Leash,
                                    ),
                                ),
                                arguments: [
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 73,
                                                },
                                            ),
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 5,
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
                                        value: 44,
                                    },
                                ),
                                refined_path: Right(
                                    Num(
                                        Int(
                                            I32,
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
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
                                        value: 44,
                                    },
                                ),
                                refined_path: Right(
                                    Num(
                                        Int(
                                            I32,
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
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
                                refined_path: Right(
                                    Num(
                                        Float(
                                            F32,
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
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
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
            DeclRegionPath::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Form(
                        FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                    ),
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        List(
                            ListFunctor,
                        ),
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
                variant_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    TypeOntologyDeclError {
                                        path: TypePath(
                                            Id {
                                                value: 82,
                                            },
                                        ),
                                    },
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
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
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
                                        value: 41,
                                    },
                                ),
                                refined_path: Right(
                                    Borrow(
                                        Leash,
                                    ),
                                ),
                                arguments: [
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 73,
                                                },
                                            ),
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 5,
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
                                        value: 54,
                                    },
                                ),
                                refined_path: Right(
                                    Num(
                                        Float(
                                            F32,
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
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        ExpectationEntry {
                            src: ExpectationSource {
                                expr_idx: 3,
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
                                    EqsRitchieCallType(
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
            DeclRegionPath::ImplBlock(
                ImplBlockId::Type(
                    TypeImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        disambiguator: 0,
                    },
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
                                    TypeOntologyDeclError {
                                        path: TypePath(
                                            Id {
                                                value: 82,
                                            },
                                        ),
                                    },
                                ),
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
                    entries: [],
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
            DeclRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `new`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Err(
                    Derived(
                        TypeError(
                            DeclarativeTypeError(
                                Derived(
                                    TypeOntologyDeclError {
                                        path: TypePath(
                                            Id {
                                                value: 82,
                                            },
                                        ),
                                    },
                                ),
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
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
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
                                        value: 41,
                                    },
                                ),
                                refined_path: Right(
                                    Borrow(
                                        Leash,
                                    ),
                                ),
                                arguments: [
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 73,
                                                },
                                            ),
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 5,
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
                                        value: 44,
                                    },
                                ),
                                refined_path: Right(
                                    Num(
                                        Int(
                                            I32,
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
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
                                        value: 44,
                                    },
                                ),
                                refined_path: Right(
                                    Num(
                                        Int(
                                            I32,
                                        ),
                                    ),
                                ),
                                arguments: [],
                                base_ty_term: Some(
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
                            },
                        ],
                    },
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
        self_ty: Some(
            EtherealTerm(`TypeOntology(mnist_classifier::line_segment_sketch::LineSegmentStroke)`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `displacement`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                    entries: [],
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
                                expr_idx: 0,
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
        self_ty: Some(
            EtherealTerm(`TypeOntology(mnist_classifier::line_segment_sketch::LineSegmentStroke)`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclRegionPath::ImplBlock(
                ImplBlockId::Type(
                    TypeImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        disambiguator: 0,
                    },
                ),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                    entries: [],
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
                                expr_idx: 0,
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
            DeclRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `concave_components`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Ok(
                    (
                        List(
                            ListFunctor,
                        ),
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
                    entries: [],
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
                                expr_idx: 0,
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
                                    EqsRitchieCallType(
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
        self_ty: Some(
            EtherealTerm(`TypeOntology(mnist_classifier::line_segment_sketch::LineSegmentSketch)`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `bounding_box`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                    entries: [],
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
                                expr_idx: 0,
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
        self_ty: Some(
            EtherealTerm(`TypeOntology(mnist_classifier::line_segment_sketch::LineSegmentSketch)`),
        ),
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclRegionPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                    ident: `new`,
                },
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                variant_and_ty_result: Ok(
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
                ],
            },
        },
        fluffy_term_region: FluffyTermRegion {
            terms: FluffyTerms {
                hollow_terms: HollowTerms {
                    entries: [],
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
                                        value: 41,
                                    },
                                ),
                                refined_path: Right(
                                    Borrow(
                                        Leash,
                                    ),
                                ),
                                arguments: [
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 73,
                                                },
                                            ),
                                        ),
                                    ),
                                ],
                                base_ty_term: Some(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 5,
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
                                        value: 54,
                                    },
                                ),
                                refined_path: Right(
                                    Num(
                                        Float(
                                            F32,
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
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
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
        self_ty: Some(
            EtherealTerm(`TypeOntology(mnist_classifier::line_segment_sketch::LineSegmentSketch)`),
        ),
    },
]