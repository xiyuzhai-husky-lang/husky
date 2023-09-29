[
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::slice::Slice`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Type`),
                        ),
                    },
                ),
            ],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`t`),
                    ),
                },
            ],
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
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Type`),
                        ),
                    },
                ),
            ],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`t`),
                    ),
                },
            ],
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
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TypeImplBlock(
                    TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `core::slice`,
                            ty_path: TypePath(`core::slice::Slice`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 1,
                                path: MajorItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 35,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Curry(
                                        EtherealTermCurry(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    6,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: ImplicitParameter {
                                    template_parameter_kind: Type {
                                        ident_token: IdentRegionalToken {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 25,
                                                    },
                                                ),
                                            ),
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Application {
                                function_sema_expr_idx: SemaExprIdx(
                                    1,
                                ),
                                argument_sema_expr_idx: SemaExprIdx(
                                    2,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                3,
                SemaExprIdx(
                    3,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Type`),
                        ),
                    },
                ),
            ],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`t`),
                    ),
                },
            ],
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::Sort,
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
                                        EtherealTerm(`covariant Type -> Type`),
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
                                                        EtherealTerm(`Type`),
                                                    ),
                                                },
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Covariant,
                                                    parameter_symbol: None,
                                                    parameter_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
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
            EtherealTerm(`Slice t`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `len`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 1,
                                path: MajorItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Type`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`t`),
                    ),
                },
            ],
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
                        FluffyTermExpectationEntry {
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
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
            EtherealTerm(`Slice t`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `swap`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 1,
                                path: MajorItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 2,
                                path: MajorItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
            (
                2,
                SemaExprIdx(
                    2,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [
                None,
                None,
            ],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Type`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`usize`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`usize`),
                        ),
                    },
                ),
            ],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`t`),
                    ),
                },
            ],
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
                        FluffyTermExpectationEntry {
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
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
                        FluffyTermExpectationEntry {
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
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
            EtherealTerm(`Slice t`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `core::slice`,
                            trai_path: TraitPath(`core::ops::IntIndex`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 3,
                                path: MajorItem(
                                    Trait(
                                        TraitPath(
                                            Id {
                                                value: 19,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: InstanceConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 5,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 4,
                                path: MajorItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 36,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Curry(
                                        EtherealTermCurry(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    12,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: ImplicitParameter {
                                    template_parameter_kind: Type {
                                        ident_token: IdentRegionalToken {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 25,
                                                    },
                                                ),
                                            ),
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Application {
                                function_sema_expr_idx: SemaExprIdx(
                                    2,
                                ),
                                argument_sema_expr_idx: SemaExprIdx(
                                    3,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
            (
                4,
                SemaExprIdx(
                    4,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Type`),
                        ),
                    },
                ),
            ],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`t`),
                    ),
                },
            ],
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
                        FluffyTermExpectationEntry {
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
                                        EtherealTerm(`Trait`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`covariant Type -> Type`),
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
                                                        EtherealTerm(`Type`),
                                                    ),
                                                },
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Covariant,
                                                    parameter_symbol: None,
                                                    parameter_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
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
            EtherealTerm(`CyclicSlice t`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TraitForTypeItem(
                    TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `core::slice`,
                                    trai_path: TraitPath(`core::ops::IntIndex`),
                                    ty_sketch: TypeSketch::Path(
                                        TypePath(`core::slice::CyclicSlice`, `Extern`),
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
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    4,
                                ),
                                inherited_symbol_idx: 1,
                                inherited_symbol_kind: TemplateParameter(
                                    Type {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Type`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`t`),
                    ),
                },
            ],
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
                        FluffyTermExpectationEntry {
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
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
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TypeImplBlock(
                    TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `core::slice`,
                            ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 1,
                                path: MajorItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 36,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Curry(
                                        EtherealTermCurry(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    6,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: ImplicitParameter {
                                    template_parameter_kind: Type {
                                        ident_token: IdentRegionalToken {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 25,
                                                    },
                                                ),
                                            ),
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                    },
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Application {
                                function_sema_expr_idx: SemaExprIdx(
                                    1,
                                ),
                                argument_sema_expr_idx: SemaExprIdx(
                                    2,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                3,
                SemaExprIdx(
                    3,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Type`),
                        ),
                    },
                ),
            ],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`t`),
                    ),
                },
            ],
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::Sort,
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
                                        EtherealTerm(`covariant Type -> Type`),
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
                                                        EtherealTerm(`Type`),
                                                    ),
                                                },
                                                variant: ExpectEqsFunctionTypeOutcomeVariant::Curry {
                                                    variance: Covariant,
                                                    parameter_symbol: None,
                                                    parameter_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: FluffyTermBase::Ethereal(
                                                            EtherealTerm(`Type`),
                                                        ),
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
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
            EtherealTerm(`CyclicSlice t`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `ilen`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 1,
                                path: MajorItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Type`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`t`),
                    ),
                },
            ],
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
                        FluffyTermExpectationEntry {
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
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
            EtherealTerm(`CyclicSlice t`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `start`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 1,
                                path: MajorItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Type`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`t`),
                    ),
                },
            ],
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
                        FluffyTermExpectationEntry {
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
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
            EtherealTerm(`CyclicSlice t`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `end`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 1,
                                path: MajorItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                ty_path_disambiguation: OntologyConstructor,
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                1,
                SemaExprIdx(
                    1,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Type`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`t`),
                    ),
                },
            ],
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
                        FluffyTermExpectationEntry {
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Type`),
                                    ),
                                },
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
            EtherealTerm(`CyclicSlice t`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `first`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    9,
                                ),
                                inherited_symbol_idx: 1,
                                inherited_symbol_kind: TemplateParameter(
                                    Type {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Prefix {
                                opr: Leash,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    8,
                                ),
                                opd_sema_expr_idx: SemaExprIdx(
                                    1,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Prefix {
                                opr: Option,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    7,
                                ),
                                opd_sema_expr_idx: SemaExprIdx(
                                    2,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                3,
                SemaExprIdx(
                    3,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Type`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`t`),
                    ),
                },
            ],
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsExactly(
                                ExpectSubtype {
                                    expected: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::Subtype(
                                            ExpectSubtypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
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
            EtherealTerm(`CyclicSlice t`),
        ),
    },
    SemaExprRegion {
        path: RegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `last`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                ),
            ),
        ),
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    9,
                                ),
                                inherited_symbol_idx: 1,
                                inherited_symbol_kind: TemplateParameter(
                                    Type {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Prefix {
                                opr: Leash,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    8,
                                ),
                                opd_sema_expr_idx: SemaExprIdx(
                                    1,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Prefix {
                                opr: Option,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    7,
                                ),
                                opd_sema_expr_idx: SemaExprIdx(
                                    2,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Category(
                                        TermCategory {
                                            universe: TermUniverse(
                                                1,
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        sema_stmt_arena: SemaStmtArena(
            Arena {
                data: [],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                3,
                SemaExprIdx(
                    3,
                ),
            ),
        ],
        pattern_expr_ty_infos: [],
        pattern_symbol_ty_infos: ArenaMap {
            data: [],
        },
        sema_expr_terms: [],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Type`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [],
        },
        symbol_terms: SymbolMap {
            inherited_symbol_map: [
                FluffyTerm {
                    place: None,
                    base: FluffyTermBase::Ethereal(
                        EtherealTerm(`t`),
                    ),
                },
            ],
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::CurryDestination(
                                ExpectCurryDestination {
                                    curry_destination: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsExactly(
                                ExpectSubtype {
                                    expected: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Category(
                                                TermCategory {
                                                    universe: TermUniverse(
                                                        1,
                                                    ),
                                                },
                                            ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Resolved(
                                    Ok(
                                        FluffyTermExpectationOutcome::Subtype(
                                            ExpectSubtypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        1,
                                    ),
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
                                        EtherealTerm(`Type`),
                                    ),
                                },
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
            EtherealTerm(`CyclicSlice t`),
        ),
    },
]