[
    SemaExprRegion {
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
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 193,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    4,
                                ),
                                inherited_symbol_idx: 2,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 12,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    1,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    5,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 156,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                },
                                method_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
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
                                                                    value: 27,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                },
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    7,
                                ),
                                ritchie_parameter_argument_matches: [],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    8,
                                ),
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
                                                    value: 27,
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
                                path_expr_idx: 1,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 2,
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
                                    Curry(
                                        EtherealTermCurry(
                                            Id {
                                                value: 12,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 193,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    11,
                                ),
                                inherited_symbol_idx: 2,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 12,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    13,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        0,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 21,
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
                                path_expr_idx: 2,
                                path: MajorItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 21,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 156,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    16,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 1,
                                },
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
                                                    value: 27,
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
                            Literal(
                                RegionalTokenIdx(
                                    18,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 27,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    7,
                                ),
                                opr: Closed(
                                    Sub,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    17,
                                ),
                                ropd: SemaExprIdx(
                                    8,
                                ),
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
                                                    value: 27,
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
                            Bracketed {
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    15,
                                ),
                                item: SemaExprIdx(
                                    9,
                                ),
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    19,
                                ),
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
                                                    value: 27,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    10,
                                ),
                                opr: As,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    20,
                                ),
                                ropd: SemaExprIdx(
                                    6,
                                ),
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
                                                    value: 21,
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
                            FnCall {
                                function_sema_expr_idx: SemaExprIdx(
                                    3,
                                ),
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    10,
                                ),
                                ritchie_parameter_argument_matches: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Hollow(
                                                    HollowTerm(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                4,
                                            ),
                                            separator: Comma(
                                                RegionalTokenIdx(
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
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 21,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                5,
                                            ),
                                            separator: Comma(
                                                RegionalTokenIdx(
                                                    14,
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
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 21,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                11,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    22,
                                ),
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
                                                    value: 4,
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
                            Block {
                                stmts: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        1..3,
                                    ),
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
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
                data: [
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        1,
                                    ),
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    2,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    12,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                13,
                SemaExprIdx(
                    13,
                ),
            ),
        ],
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
                                            value: 27,
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
                                                    value: 27,
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
        sema_expr_terms: [
            (
                SemaExprIdx(
                    6,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 21,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    5,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                ISize(
                                    TermISizeLiteral(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    8,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                USize(
                                    TermUSizeLiteral(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ],
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
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Slice t`),
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
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    3,
                                ),
                                hole_kind: ImplicitType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Symbol(
                                                EtherealTermSymbol(
                                                    Id {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`t`),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 35,
                                    },
                                ),
                                refined_path: Left(
                                    Arr(
                                        Slice,
                                    ),
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
                                EtherealTerm(`Slice t`),
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
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
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
                                                                    value: 21,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
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
                                                                    value: 21,
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
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`fn(move  Slice t,  isize,  isize) -> unit`),
                            ),
                        },
                    ],
                    first_unresolved_term_idx: 1,
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
                                        EtherealTerm(`Slice t`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
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
                                        EtherealTerm(`usize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::AnyDerived,
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
                                        EtherealTerm(`(independent variable_ad_hoc_fmt: Type) -> fn(move  Slice variable_ad_hoc_fmt,  isize,  isize) -> unit`),
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
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`unit`),
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
                                                                            1,
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
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    Id {
                                                                                        value: 21,
                                                                                    },
                                                                                ),
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
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    Id {
                                                                                        value: 21,
                                                                                    },
                                                                                ),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
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
                                        EtherealTerm(`Slice t`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsExactly(
                                ExpectSubtype {
                                    expected: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expectation(
                                        4,
                                    ),
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`t`),
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        0,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 10,
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
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`usize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
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
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`usize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ExplicitlyConvertible(
                                ExpectCasting {
                                    destination: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`usize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ExplicitlyConvertible(
                                ExpectCasting {
                                    destination: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`usize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
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
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
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
                                        EtherealTerm(`unit`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
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
                                        EtherealTerm(`unit`),
                                    ),
                                },
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
    SemaExprRegion {
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
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 195,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    2,
                                ),
                                inherited_symbol_idx: 3,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 195,
                                            },
                                        ),
                                    ),
                                },
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
                                                    value: 21,
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
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 196,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    4,
                                ),
                                inherited_symbol_idx: 4,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 196,
                                            },
                                        ),
                                    ),
                                },
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
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    1,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    3,
                                ),
                                ropd: SemaExprIdx(
                                    2,
                                ),
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
                                                    value: 2,
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
                                path_expr_idx: 1,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 3,
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
                                    Curry(
                                        EtherealTermCurry(
                                            Id {
                                                value: 13,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 193,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    11,
                                ),
                                inherited_symbol_idx: 2,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 12,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 195,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    13,
                                ),
                                inherited_symbol_idx: 3,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 195,
                                            },
                                        ),
                                    ),
                                },
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
                                                    value: 21,
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
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 196,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    15,
                                ),
                                inherited_symbol_idx: 4,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 196,
                                            },
                                        ),
                                    ),
                                },
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
                                                    value: 21,
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
                            FnCall {
                                function_sema_expr_idx: SemaExprIdx(
                                    4,
                                ),
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    10,
                                ),
                                ritchie_parameter_argument_matches: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Hollow(
                                                    HollowTerm(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                5,
                                            ),
                                            separator: Comma(
                                                RegionalTokenIdx(
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
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 21,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                6,
                                            ),
                                            separator: Comma(
                                                RegionalTokenIdx(
                                                    14,
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
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 21,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                7,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    16,
                                ),
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
                                                    value: 21,
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
                                path_expr_idx: 2,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 2,
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
                                    Curry(
                                        EtherealTermCurry(
                                            Id {
                                                value: 12,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 193,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    19,
                                ),
                                inherited_symbol_idx: 2,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 12,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 195,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    21,
                                ),
                                inherited_symbol_idx: 3,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 195,
                                            },
                                        ),
                                    ),
                                },
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
                                                    value: 21,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 197,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    23,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 1,
                                },
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
                                                    value: 21,
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
                            Literal(
                                RegionalTokenIdx(
                                    25,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    12,
                                ),
                                opr: Closed(
                                    Sub,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    24,
                                ),
                                ropd: SemaExprIdx(
                                    13,
                                ),
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
                                                    value: 21,
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
                            FnCall {
                                function_sema_expr_idx: SemaExprIdx(
                                    9,
                                ),
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    18,
                                ),
                                ritchie_parameter_argument_matches: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Hollow(
                                                    HollowTerm(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                10,
                                            ),
                                            separator: Comma(
                                                RegionalTokenIdx(
                                                    20,
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
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 21,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                11,
                                            ),
                                            separator: Comma(
                                                RegionalTokenIdx(
                                                    22,
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
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 21,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                14,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    26,
                                ),
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
                                                    value: 4,
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
                                path_expr_idx: 3,
                                path: MajorItem(
                                    Fugitive(
                                        FugitivePath(
                                            Id {
                                                value: 2,
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
                                    Curry(
                                        EtherealTermCurry(
                                            Id {
                                                value: 12,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 193,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    29,
                                ),
                                inherited_symbol_idx: 2,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 12,
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
                                            value: 197,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    31,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 1,
                                },
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
                                                    value: 21,
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
                            Literal(
                                RegionalTokenIdx(
                                    33,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    18,
                                ),
                                opr: Closed(
                                    Add,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    32,
                                ),
                                ropd: SemaExprIdx(
                                    19,
                                ),
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
                                                    value: 21,
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
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 196,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    35,
                                ),
                                inherited_symbol_idx: 4,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 196,
                                            },
                                        ),
                                    ),
                                },
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
                                                    value: 21,
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
                            FnCall {
                                function_sema_expr_idx: SemaExprIdx(
                                    16,
                                ),
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    28,
                                ),
                                ritchie_parameter_argument_matches: [
                                    Regular(
                                        FluffyTermRitchieRegularParameter {
                                            contract: Move,
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Hollow(
                                                    HollowTerm(
                                                        7,
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                17,
                                            ),
                                            separator: Comma(
                                                RegionalTokenIdx(
                                                    30,
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
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 21,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                20,
                                            ),
                                            separator: Comma(
                                                RegionalTokenIdx(
                                                    34,
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
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 21,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                21,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    36,
                                ),
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
                                                    value: 4,
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
                            Block {
                                stmts: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        4..5,
                                    ),
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
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
                data: [
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        1,
                                    ),
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        8,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    8,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    15,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    22,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            IfElse {
                                sema_if_branch: SemaIfBranch {
                                    if_token: IfRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        3,
                                    ),
                                    eol_colon: Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    ),
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            1..4,
                                        ),
                                    ),
                                },
                                sema_elif_branches: [],
                                sema_else_branch: None,
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                23,
                SemaExprIdx(
                    23,
                ),
            ),
        ],
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
                                            value: 21,
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
                                                    value: 21,
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
        sema_expr_terms: [
            (
                SemaExprIdx(
                    13,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                ISize(
                                    TermISizeLiteral(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    19,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                ISize(
                                    TermISizeLiteral(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ],
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
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Slice t`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`isize`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`isize`),
                        ),
                    },
                ),
            ],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`isize`),
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
                    entries: [
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    4,
                                ),
                                hole_kind: ImplicitType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Symbol(
                                                EtherealTermSymbol(
                                                    Id {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`t`),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 35,
                                    },
                                ),
                                refined_path: Left(
                                    Arr(
                                        Slice,
                                    ),
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
                                EtherealTerm(`Slice t`),
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
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
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
                                                                    value: 21,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
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
                                                                    value: 21,
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
                                                        value: 21,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`fn(move  Slice t,  isize,  isize) -> isize`),
                            ),
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    10,
                                ),
                                hole_kind: ImplicitType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Symbol(
                                                EtherealTermSymbol(
                                                    Id {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`t`),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 35,
                                    },
                                ),
                                refined_path: Left(
                                    Arr(
                                        Slice,
                                    ),
                                ),
                                arguments: [
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`Slice t`),
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
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
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
                                                                    value: 21,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
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
                                                                    value: 21,
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
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`fn(move  Slice t,  isize,  isize) -> unit`),
                            ),
                        },
                        HollowTermEntry {
                            data: Hole {
                                hole_source: Expectation(
                                    18,
                                ),
                                hole_kind: ImplicitType,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Symbol(
                                                EtherealTermSymbol(
                                                    Id {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                constraints: [],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`t`),
                            ),
                        },
                        HollowTermEntry {
                            data: TypeOntology {
                                path: TypePath(
                                    Id {
                                        value: 35,
                                    },
                                ),
                                refined_path: Left(
                                    Arr(
                                        Slice,
                                    ),
                                ),
                                arguments: [
                                    FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                6,
                                            ),
                                        ),
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`Slice t`),
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
                                                        7,
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
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
                                                                    value: 21,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
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
                                                                    value: 21,
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
                                                        value: 4,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`fn(move  Slice t,  isize,  isize) -> unit`),
                            ),
                        },
                    ],
                    first_unresolved_term_idx: 7,
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
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
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
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
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
                        FluffyTermExpectationEntry {
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`(independent variable_ad_hoc_fmt: Type) -> fn(move  Slice variable_ad_hoc_fmt,  isize,  isize) -> isize`),
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
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`isize`),
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
                                                                            1,
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
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    Id {
                                                                                        value: 21,
                                                                                    },
                                                                                ),
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
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    Id {
                                                                                        value: 21,
                                                                                    },
                                                                                ),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                1,
                                            ),
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
                                        EtherealTerm(`Slice t`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsExactly(
                                ExpectSubtype {
                                    expected: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
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
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`t`),
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 7,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 7,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 9,
                                src: ExpectationSource {
                                    expr_idx: 8,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::TypeOntology,
                                },
                            ),
                            meta: ExpectationState {
                                idx: 10,
                                src: ExpectationSource {
                                    expr_idx: 9,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`(independent variable_ad_hoc_fmt: Type) -> fn(move  Slice variable_ad_hoc_fmt,  isize,  isize) -> unit`),
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
                                                                    3,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`unit`),
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
                                                                            4,
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
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    Id {
                                                                                        value: 21,
                                                                                    },
                                                                                ),
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
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    Id {
                                                                                        value: 21,
                                                                                    },
                                                                                ),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                4,
                                            ),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 11,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Slice t`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsExactly(
                                ExpectSubtype {
                                    expected: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 10,
                                    kind: Expectation(
                                        11,
                                    ),
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`t`),
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
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 11,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 14,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 13,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 14,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
                                        ),
                                    },
                                },
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
                                        EtherealTerm(`unit`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsFunctionType(
                                ExpectEqsFunctionType {
                                    final_destination: FinalDestination::AnyDerived,
                                },
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
                                        EtherealTerm(`(independent variable_ad_hoc_fmt: Type) -> fn(move  Slice variable_ad_hoc_fmt,  isize,  isize) -> unit`),
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
                                                                    6,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: FluffyTermBase::Ethereal(
                                                        EtherealTerm(`unit`),
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
                                                                            7,
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
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    Id {
                                                                                        value: 21,
                                                                                    },
                                                                                ),
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
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    Id {
                                                                                        value: 21,
                                                                                    },
                                                                                ),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Hollow(
                                            HollowTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                },
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
                                        EtherealTerm(`Slice t`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsExactly(
                                ExpectSubtype {
                                    expected: FluffyTerm {
                                        place: None,
                                        base: Hollow(
                                            HollowTerm(
                                                6,
                                            ),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 17,
                                    kind: Expectation(
                                        19,
                                    ),
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`t`),
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
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 18,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
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
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
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
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
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
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
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
                                        EtherealTerm(`unit`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
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
                                        EtherealTerm(`unit`),
                                    ),
                                },
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
    SemaExprRegion {
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
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 196,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    4,
                                ),
                                inherited_symbol_idx: 4,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 196,
                                            },
                                        ),
                                    ),
                                },
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
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    2,
                                ),
                                opr: As,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    5,
                                ),
                                ropd: SemaExprIdx(
                                    1,
                                ),
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
                                                    value: 27,
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
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 195,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    11,
                                ),
                                inherited_symbol_idx: 3,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 195,
                                            },
                                        ),
                                    ),
                                },
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
                                                    value: 21,
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
                            Literal(
                                RegionalTokenIdx(
                                    13,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    4,
                                ),
                                opr: Closed(
                                    Sub,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    12,
                                ),
                                ropd: SemaExprIdx(
                                    5,
                                ),
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
                                                    value: 21,
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
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 196,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    18,
                                ),
                                inherited_symbol_idx: 4,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 196,
                                            },
                                        ),
                                    ),
                                },
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
                                                    value: 21,
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
                            Literal(
                                RegionalTokenIdx(
                                    20,
                                ),
                                Bool(
                                    True,
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
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
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 200,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    22,
                                ),
                                current_symbol_idx: 2,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 2,
                                },
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
                                                    value: 21,
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
                            Literal(
                                RegionalTokenIdx(
                                    24,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    9,
                                ),
                                opr: AssignClosed(
                                    Add,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    23,
                                ),
                                ropd: SemaExprIdx(
                                    10,
                                ),
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
                                                    value: 4,
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
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 193,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    26,
                                ),
                                inherited_symbol_idx: 2,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 12,
                                            },
                                        ),
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
                    SemaExprEntry {
                        data_result: Ok(
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 200,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    28,
                                ),
                                current_symbol_idx: 2,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 2,
                                },
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
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    14,
                                ),
                                opr: As,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    29,
                                ),
                                ropd: SemaExprIdx(
                                    13,
                                ),
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
                                                    value: 27,
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
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    12,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    27,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            15,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    31,
                                ),
                                index_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: Int {
                                        element_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                ),
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
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 193,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    33,
                                ),
                                inherited_symbol_idx: 2,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 12,
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
                                            value: 199,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    35,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 1,
                                },
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
                                                    value: 27,
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
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    17,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    34,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            18,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    36,
                                ),
                                index_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: Int {
                                        element_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                ),
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
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    16,
                                ),
                                opr: Comparison(
                                    Less,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    32,
                                ),
                                ropd: SemaExprIdx(
                                    19,
                                ),
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
                                                    value: 2,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 200,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    38,
                                ),
                                current_symbol_idx: 2,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 2,
                                },
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
                                                    value: 21,
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
                            Literal(
                                RegionalTokenIdx(
                                    40,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    21,
                                ),
                                opr: AssignClosed(
                                    Add,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    39,
                                ),
                                ropd: SemaExprIdx(
                                    22,
                                ),
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
                                                    value: 4,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 201,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    41,
                                ),
                                current_symbol_idx: 3,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 3,
                                },
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
                                                    value: 21,
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
                            Literal(
                                RegionalTokenIdx(
                                    43,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    24,
                                ),
                                opr: AssignClosed(
                                    Sub,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    42,
                                ),
                                ropd: SemaExprIdx(
                                    25,
                                ),
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
                                                    value: 4,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 201,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    45,
                                ),
                                current_symbol_idx: 3,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 3,
                                },
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
                                                    value: 21,
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
                            Literal(
                                RegionalTokenIdx(
                                    47,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        0,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    27,
                                ),
                                opr: Comparison(
                                    Geq,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    46,
                                ),
                                ropd: SemaExprIdx(
                                    28,
                                ),
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
                                                    value: 2,
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
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 193,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    49,
                                ),
                                inherited_symbol_idx: 2,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 12,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 3,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 201,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    51,
                                ),
                                current_symbol_idx: 3,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 3,
                                },
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
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    32,
                                ),
                                opr: As,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    52,
                                ),
                                ropd: SemaExprIdx(
                                    31,
                                ),
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
                                                    value: 27,
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
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    30,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    50,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            33,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    54,
                                ),
                                index_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: Int {
                                        element_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                ),
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
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 193,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    56,
                                ),
                                inherited_symbol_idx: 2,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 12,
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
                                            value: 199,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    58,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 1,
                                },
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
                                                    value: 27,
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
                            Index {
                                owner_sema_expr_idx: SemaExprIdx(
                                    35,
                                ),
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    57,
                                ),
                                index_sema_list_items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            36,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    59,
                                ),
                                index_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
                                    signature: Int {
                                        element_ty: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Symbol(
                                                    EtherealTermSymbol(
                                                        Id {
                                                            value: 1,
                                                        },
                                                    ),
                                                ),
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
                                    Symbol(
                                        EtherealTermSymbol(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Binary {
                                lopd: SemaExprIdx(
                                    34,
                                ),
                                opr: Comparison(
                                    Greater,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    55,
                                ),
                                ropd: SemaExprIdx(
                                    37,
                                ),
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
                                                    value: 2,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    29,
                                ),
                                opr: ShortCircuitLogic(
                                    And,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    48,
                                ),
                                ropd: SemaExprIdx(
                                    38,
                                ),
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
                                                    value: 2,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 201,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    61,
                                ),
                                current_symbol_idx: 3,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 3,
                                },
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
                                                    value: 21,
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
                            Literal(
                                RegionalTokenIdx(
                                    63,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    40,
                                ),
                                opr: AssignClosed(
                                    Sub,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    62,
                                ),
                                ropd: SemaExprIdx(
                                    41,
                                ),
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
                                                    value: 4,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 200,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    65,
                                ),
                                current_symbol_idx: 2,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 2,
                                },
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
                                                    value: 21,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 201,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    67,
                                ),
                                current_symbol_idx: 3,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 3,
                                },
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
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    43,
                                ),
                                opr: Comparison(
                                    Geq,
                                ),
                                opr_regional_token_idx: RegionalTokenIdx(
                                    66,
                                ),
                                ropd: SemaExprIdx(
                                    44,
                                ),
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
                                                    value: 2,
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
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 193,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    72,
                                ),
                                inherited_symbol_idx: 2,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 12,
                                            },
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 200,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    76,
                                ),
                                current_symbol_idx: 2,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 2,
                                },
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
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    48,
                                ),
                                opr: As,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    77,
                                ),
                                ropd: SemaExprIdx(
                                    47,
                                ),
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
                                                    value: 27,
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
                                path_expr_idx: 5,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 201,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    80,
                                ),
                                current_symbol_idx: 3,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 3,
                                },
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
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    51,
                                ),
                                opr: As,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    81,
                                ),
                                ropd: SemaExprIdx(
                                    50,
                                ),
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
                                                    value: 27,
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
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    46,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    73,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 157,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        74,
                                    ),
                                },
                                method_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
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
                                                                                value: 27,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ),
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
                                                                                value: 27,
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
                                                                    value: 4,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                },
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    75,
                                ),
                                ritchie_parameter_argument_matches: [
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
                                                                    value: 27,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                49,
                                            ),
                                            separator: Comma(
                                                RegionalTokenIdx(
                                                    79,
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
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 27,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                52,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    83,
                                ),
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
                                                    value: 4,
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
                            InheritedSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 193,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    84,
                                ),
                                inherited_symbol_idx: 2,
                                inherited_symbol_kind: ParenateParameter {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 12,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            PrincipalEntityPath {
                                path_expr_idx: 6,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 200,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    88,
                                ),
                                current_symbol_idx: 2,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 2,
                                },
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
                                                    value: 21,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    56,
                                ),
                                opr: As,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    89,
                                ),
                                ropd: SemaExprIdx(
                                    55,
                                ),
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
                                                    value: 27,
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
                                path_expr_idx: 7,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 199,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    92,
                                ),
                                current_symbol_idx: 1,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 1,
                                },
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
                                                    value: 27,
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
                            Binary {
                                lopd: SemaExprIdx(
                                    59,
                                ),
                                opr: As,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    93,
                                ),
                                ropd: SemaExprIdx(
                                    58,
                                ),
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
                                                    value: 27,
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
                            MethodFnCall {
                                self_argument_sema_expr_idx: SemaExprIdx(
                                    54,
                                ),
                                dot_regional_token_idx: RegionalTokenIdx(
                                    85,
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 157,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        86,
                                    ),
                                },
                                method_dynamic_dispatch: FluffyDynamicDispatch {
                                    indirections: FluffyTermDynamicDispatchIndirections {
                                        initial_place: Transient,
                                        indirections: [],
                                        final_place: Transient,
                                    },
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
                                                                                value: 27,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ),
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
                                                                                value: 27,
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
                                                                    value: 4,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                },
                                template_arguments: None,
                                lpar_regional_token_idx: RegionalTokenIdx(
                                    87,
                                ),
                                ritchie_parameter_argument_matches: [
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
                                                                    value: 27,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                57,
                                            ),
                                            separator: Comma(
                                                RegionalTokenIdx(
                                                    91,
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
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                Id {
                                                                    value: 27,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        SemaRegularCallListItem {
                                            argument_expr_idx: SemaExprIdx(
                                                60,
                                            ),
                                            separator: None,
                                        },
                                    ),
                                ],
                                rpar_regional_token_idx: RegionalTokenIdx(
                                    95,
                                ),
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
                                                    value: 4,
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
                            CurrentSymbol {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 200,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    96,
                                ),
                                current_symbol_idx: 2,
                                current_symbol_kind: LetVariable {
                                    pattern_symbol_idx: 2,
                                },
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
                                                    value: 21,
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
                            Block {
                                stmts: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        10..16,
                                    ),
                                ),
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
                                                    value: 21,
                                                },
                                            ),
                                        ),
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
                data: [
                    SemaStmtEntry {
                        data_result: Ok(
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    23,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    42,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Break {
                                break_token: BreakRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        69,
                                    ),
                                },
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
                                                    value: 3,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    53,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    11,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            While {
                                while_token: WhileRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    20,
                                ),
                                eol_colon: Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            37,
                                        ),
                                    },
                                ),
                                block: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        1..2,
                                    ),
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    26,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            While {
                                while_token: WhileRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        44,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    39,
                                ),
                                eol_colon: Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            60,
                                        ),
                                    },
                                ),
                                block: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        2..3,
                                    ),
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            IfElse {
                                sema_if_branch: SemaIfBranch {
                                    if_token: IfRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            64,
                                        ),
                                    },
                                    condition: SemaExprIdx(
                                        45,
                                    ),
                                    eol_colon: Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                68,
                                            ),
                                        },
                                    ),
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            3..4,
                                        ),
                                    ),
                                },
                                sema_elif_branches: [],
                                sema_else_branch: Some(
                                    SemaElseBranch {
                                        else_token: ElseRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                70,
                                            ),
                                        },
                                        eol_colon: Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    71,
                                                ),
                                            },
                                        ),
                                        stmts: SemaStmtIdxRange(
                                            ArenaIdxRange(
                                                4..5,
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
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        1,
                                    ),
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    3,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        2,
                                    ),
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        10,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    6,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        3,
                                    ),
                                    variables: ArenaIdxRange(
                                        3..4,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        17,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    7,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            While {
                                while_token: WhileRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                },
                                condition: SemaExprIdx(
                                    8,
                                ),
                                eol_colon: Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            21,
                                        ),
                                    },
                                ),
                                block: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        5..10,
                                    ),
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    61,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaStmtEntry {
                        data_result: Ok(
                            Eval {
                                sema_expr_idx: SemaExprIdx(
                                    62,
                                ),
                                eol_semicolon: Ok(
                                    None,
                                ),
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
                                                    value: 21,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                63,
                SemaExprIdx(
                    63,
                ),
            ),
        ],
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
                                            value: 27,
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
                                            value: 21,
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
                                            value: 21,
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
                                                    value: 27,
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
                                                    value: 21,
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
                                                    value: 21,
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
        sema_expr_terms: [
            (
                SemaExprIdx(
                    1,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 27,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    13,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 27,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    31,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 27,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    47,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 27,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    50,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 27,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    55,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 27,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    58,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 27,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    5,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                ISize(
                                    TermISizeLiteral(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    8,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                Bool(
                                    true,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    10,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                ISize(
                                    TermISizeLiteral(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    22,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                ISize(
                                    TermISizeLiteral(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    25,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                ISize(
                                    TermISizeLiteral(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    28,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                ISize(
                                    TermISizeLiteral(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    41,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                ISize(
                                    TermISizeLiteral(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ],
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
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Slice t`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`isize`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`isize`),
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
                            EtherealTerm(`isize`),
                        ),
                    },
                ),
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`isize`),
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
                                        0,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 1,
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ExplicitlyConvertible(
                                ExpectCasting {
                                    destination: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
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
                                        EtherealTerm(`usize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
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
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
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
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
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
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
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
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
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
                        FluffyTermExpectationEntry {
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
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
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
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
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
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 12,
                                src: ExpectationSource {
                                    expr_idx: 12,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Slice t`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        0,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 14,
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
                            expectation: Expectation::ExplicitlyConvertible(
                                ExpectCasting {
                                    destination: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
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
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 15,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`usize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 18,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`t`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 17,
                                src: ExpectationSource {
                                    expr_idx: 16,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Slice t`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 18,
                                src: ExpectationSource {
                                    expr_idx: 17,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`usize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`t`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 19,
                                src: ExpectationSource {
                                    expr_idx: 19,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`t`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 20,
                                src: ExpectationSource {
                                    expr_idx: 20,
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 21,
                                src: ExpectationSource {
                                    expr_idx: 21,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 22,
                                src: ExpectationSource {
                                    expr_idx: 22,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 23,
                                src: ExpectationSource {
                                    expr_idx: 23,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 24,
                                src: ExpectationSource {
                                    expr_idx: 24,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 25,
                                src: ExpectationSource {
                                    expr_idx: 25,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 26,
                                src: ExpectationSource {
                                    expr_idx: 26,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 27,
                                src: ExpectationSource {
                                    expr_idx: 27,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 28,
                                src: ExpectationSource {
                                    expr_idx: 28,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
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
                                idx: 29,
                                src: ExpectationSource {
                                    expr_idx: 37,
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 30,
                                src: ExpectationSource {
                                    expr_idx: 29,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Slice t`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        0,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 31,
                                src: ExpectationSource {
                                    expr_idx: 31,
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
                            expectation: Expectation::ExplicitlyConvertible(
                                ExpectCasting {
                                    destination: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 32,
                                src: ExpectationSource {
                                    expr_idx: 30,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 33,
                                src: ExpectationSource {
                                    expr_idx: 32,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`usize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 34,
                                src: ExpectationSource {
                                    expr_idx: 35,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`t`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 35,
                                src: ExpectationSource {
                                    expr_idx: 33,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Slice t`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 36,
                                src: ExpectationSource {
                                    expr_idx: 34,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`usize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`t`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 37,
                                src: ExpectationSource {
                                    expr_idx: 36,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`t`),
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
                        FluffyTermExpectationEntry {
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
                                idx: 38,
                                src: ExpectationSource {
                                    expr_idx: 38,
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
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
                                        FluffyTermExpectationOutcome::ConditionType(
                                            ExpectConditionTypeOutcome,
                                        ),
                                    ),
                                ),
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 40,
                                src: ExpectationSource {
                                    expr_idx: 40,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 41,
                                src: ExpectationSource {
                                    expr_idx: 41,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 42,
                                src: ExpectationSource {
                                    expr_idx: 42,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 43,
                                src: ExpectationSource {
                                    expr_idx: 43,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 44,
                                src: ExpectationSource {
                                    expr_idx: 44,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ConditionType(
                                ExpectConditionType,
                            ),
                            meta: ExpectationState {
                                idx: 45,
                                src: ExpectationSource {
                                    expr_idx: 45,
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 46,
                                src: ExpectationSource {
                                    expr_idx: 46,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Slice t`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        0,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 47,
                                src: ExpectationSource {
                                    expr_idx: 48,
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
                            expectation: Expectation::ExplicitlyConvertible(
                                ExpectCasting {
                                    destination: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 48,
                                src: ExpectationSource {
                                    expr_idx: 47,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
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
                                idx: 49,
                                src: ExpectationSource {
                                    expr_idx: 49,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`usize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        0,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 50,
                                src: ExpectationSource {
                                    expr_idx: 51,
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
                            expectation: Expectation::ExplicitlyConvertible(
                                ExpectCasting {
                                    destination: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 51,
                                src: ExpectationSource {
                                    expr_idx: 50,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
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
                                idx: 52,
                                src: ExpectationSource {
                                    expr_idx: 52,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`usize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 53,
                                src: ExpectationSource {
                                    expr_idx: 53,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyOriginal(
                                ExpectAnyOriginal,
                            ),
                            meta: ExpectationState {
                                idx: 54,
                                src: ExpectationSource {
                                    expr_idx: 54,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Slice t`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        0,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 55,
                                src: ExpectationSource {
                                    expr_idx: 56,
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
                            expectation: Expectation::ExplicitlyConvertible(
                                ExpectCasting {
                                    destination: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 56,
                                src: ExpectationSource {
                                    expr_idx: 55,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
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
                                idx: 57,
                                src: ExpectationSource {
                                    expr_idx: 57,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`usize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::EqsSort(
                                ExpectEqsCategory {
                                    smallest_universe: TermUniverse(
                                        0,
                                    ),
                                },
                            ),
                            meta: ExpectationState {
                                idx: 58,
                                src: ExpectationSource {
                                    expr_idx: 59,
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
                            expectation: Expectation::ExplicitlyConvertible(
                                ExpectCasting {
                                    destination: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`usize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 59,
                                src: ExpectationSource {
                                    expr_idx: 58,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`usize`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
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
                                idx: 60,
                                src: ExpectationSource {
                                    expr_idx: 60,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`usize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: None,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`unit`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 61,
                                src: ExpectationSource {
                                    expr_idx: 61,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 62,
                                src: ExpectationSource {
                                    expr_idx: 62,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`isize`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 63,
                                src: ExpectationSource {
                                    expr_idx: 63,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`isize`),
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
            EtherealTerm(`isize`),
        ),
        self_ty: None,
    },
    SemaExprRegion {
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
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            VecFunctor {
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    5,
                                ),
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    6,
                                ),
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
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    10,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        4,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
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
                            Literal(
                                RegionalTokenIdx(
                                    12,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        65,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
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
                            Literal(
                                RegionalTokenIdx(
                                    14,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        2,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
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
                            Literal(
                                RegionalTokenIdx(
                                    17,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        31,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Prefix {
                                opr: Minus,
                                opr_regional_token_idx: RegionalTokenIdx(
                                    16,
                                ),
                                opd_sema_expr_idx: SemaExprIdx(
                                    7,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Hollow(
                                    HollowTerm(
                                        0,
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    19,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        0,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
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
                            Literal(
                                RegionalTokenIdx(
                                    21,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        99,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
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
                            Literal(
                                RegionalTokenIdx(
                                    23,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        2,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
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
                            Literal(
                                RegionalTokenIdx(
                                    25,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        83,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
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
                            Literal(
                                RegionalTokenIdx(
                                    27,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        782,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
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
                            Literal(
                                RegionalTokenIdx(
                                    29,
                                ),
                                Integer(
                                    UnspecifiedRegular(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                Id {
                                                    value: 18,
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
                            NewList {
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    9,
                                ),
                                items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            4,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                11,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            5,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                13,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            6,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                15,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            8,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                18,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            9,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                20,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            10,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                22,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            11,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                24,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            12,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                26,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            13,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                28,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            14,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    30,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 14,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Block {
                                stmts: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        1..2,
                                    ),
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
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
                data: [
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        1,
                                    ),
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon_token: Some(
                                        ColonRegionalToken(
                                            RegionalTokenIdx(
                                                4,
                                            ),
                                        ),
                                    ),
                                    ty_sema_expr_idx: Some(
                                        SemaExprIdx(
                                            3,
                                        ),
                                    ),
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        8,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    15,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                16,
                SemaExprIdx(
                    16,
                ),
            ),
        ],
        pattern_expr_ty_infos: [
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 14,
                                    },
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
                                                value: 14,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            ],
        },
        sema_expr_terms: [
            (
                SemaExprIdx(
                    1,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 34,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    2,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            EntityPath(
                                TypeOntology(
                                    TypePath(
                                        Id {
                                            value: 18,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    3,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    4,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    4,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    5,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    65,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    6,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    2,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    7,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    31,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    9,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    0,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    10,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    99,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    11,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    2,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    12,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    83,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    13,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    782,
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    14,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                I32(
                                    1,
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Vec i32`),
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
                                    7,
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
                                                            value: 18,
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
                                                                value: 18,
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
                                EtherealTerm(`i32`),
                            ),
                        },
                    ],
                    first_unresolved_term_idx: 1,
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
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
                        FluffyTermExpectationEntry {
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
                                    base: FluffyTermBase::Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 8,
                                src: ExpectationSource {
                                    expr_idx: 8,
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                },
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 13,
                                src: ExpectationSource {
                                    expr_idx: 13,
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`i32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 14,
                                src: ExpectationSource {
                                    expr_idx: 14,
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::ImplicitlyConvertible(
                                ExpectCoersion {
                                    contract: Move,
                                    ty_expected: FluffyTerm {
                                        place: None,
                                        base: FluffyTermBase::Ethereal(
                                            EtherealTerm(`Vec i32`),
                                        ),
                                    },
                                },
                            ),
                            meta: ExpectationState {
                                idx: 15,
                                src: ExpectationSource {
                                    expr_idx: 15,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Vec i32`),
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
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
                            ),
                            meta: ExpectationState {
                                idx: 16,
                                src: ExpectationSource {
                                    expr_idx: 16,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`unit`),
                                    ),
                                },
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
    SemaExprRegion {
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
        sema_expr_arena: SemaExprArena(
            Arena {
                data: [
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    6,
                                ),
                                String(
                                    StringLiteralData(
                                        Id {
                                            value: 18,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 11,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    8,
                                ),
                                String(
                                    StringLiteralData(
                                        Id {
                                            value: 19,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 11,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    10,
                                ),
                                String(
                                    StringLiteralData(
                                        Id {
                                            value: 20,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 11,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    12,
                                ),
                                String(
                                    StringLiteralData(
                                        Id {
                                            value: 21,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 11,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    14,
                                ),
                                String(
                                    StringLiteralData(
                                        Id {
                                            value: 22,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 11,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Literal(
                                RegionalTokenIdx(
                                    16,
                                ),
                                String(
                                    StringLiteralData(
                                        Id {
                                            value: 23,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 11,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            NewList {
                                lbox_regional_token_idx: RegionalTokenIdx(
                                    5,
                                ),
                                items: [
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            1,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                7,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            2,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                9,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            3,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                11,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            4,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                13,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            5,
                                        ),
                                        comma_regional_token_idx: Some(
                                            RegionalTokenIdx(
                                                15,
                                            ),
                                        ),
                                    },
                                    SemaCommaListItem {
                                        sema_expr_idx: SemaExprIdx(
                                            6,
                                        ),
                                        comma_regional_token_idx: None,
                                    },
                                ],
                                rbox_regional_token_idx: RegionalTokenIdx(
                                    17,
                                ),
                            },
                        ),
                        ty_result: Ok(
                            FluffyTerm {
                                place: None,
                                base: Ethereal(
                                    Application(
                                        EtherealTermApplication(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                    SemaExprEntry {
                        data_result: Ok(
                            Block {
                                stmts: SemaStmtIdxRange(
                                    ArenaIdxRange(
                                        1..2,
                                    ),
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
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
                data: [
                    SemaStmtEntry {
                        data_result: Ok(
                            Let {
                                let_token: LetRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                                let_pattern_sema_obelisk: LetPatternSemaObelisk {
                                    syn_pattern_root: SynPatternRoot(
                                        1,
                                    ),
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon_token: None,
                                    ty_sema_expr_idx: None,
                                },
                                eq_token: EqRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                                initial_value_sema_expr_idx: SemaExprIdx(
                                    7,
                                ),
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
                                                    value: 4,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ),
        syn_expr_root_sema_expr_idx_table: [
            (
                8,
                SemaExprIdx(
                    8,
                ),
            ),
        ],
        pattern_expr_ty_infos: [
            PatternExprTypeInfo {
                ty: Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Application(
                                EtherealTermApplication(
                                    Id {
                                        value: 15,
                                    },
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
                                                value: 15,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            ],
        },
        sema_expr_terms: [
            (
                SemaExprIdx(
                    1,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                String(
                                    StringLiteralData(
                                        Id {
                                            value: 18,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    2,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                String(
                                    StringLiteralData(
                                        Id {
                                            value: 19,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    3,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                String(
                                    StringLiteralData(
                                        Id {
                                            value: 20,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    4,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                String(
                                    StringLiteralData(
                                        Id {
                                            value: 21,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    5,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                String(
                                    StringLiteralData(
                                        Id {
                                            value: 22,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
            (
                SemaExprIdx(
                    6,
                ),
                Ok(
                    FluffyTerm {
                        place: None,
                        base: Ethereal(
                            Literal(
                                String(
                                    StringLiteralData(
                                        Id {
                                            value: 23,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ],
        symbol_tys: SymbolMap {
            inherited_symbol_map: [],
            current_symbol_map: [
                SymbolType(
                    FluffyTerm {
                        place: None,
                        base: FluffyTermBase::Ethereal(
                            EtherealTerm(`Vec Ref 'static str`),
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
                                    7,
                                ),
                                hole_kind: Any,
                                fill: Some(
                                    FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 11,
                                                    },
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
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 11,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ],
                            },
                            resolve_progress: HollowTermResolveProgressBuf::ResolvedEthereal(
                                EtherealTerm(`Ref 'static str`),
                            ),
                        },
                    ],
                    first_unresolved_term_idx: 1,
                },
            },
            expectations: Expectations {
                arena: Arena {
                    data: [
                        FluffyTermExpectationEntry {
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
                                idx: 1,
                                src: ExpectationSource {
                                    expr_idx: 1,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Ref 'static str`),
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
                        FluffyTermExpectationEntry {
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
                                idx: 2,
                                src: ExpectationSource {
                                    expr_idx: 2,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Ref 'static str`),
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
                        FluffyTermExpectationEntry {
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
                                idx: 3,
                                src: ExpectationSource {
                                    expr_idx: 3,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Ref 'static str`),
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
                        FluffyTermExpectationEntry {
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
                                idx: 4,
                                src: ExpectationSource {
                                    expr_idx: 4,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Ref 'static str`),
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
                        FluffyTermExpectationEntry {
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
                                idx: 5,
                                src: ExpectationSource {
                                    expr_idx: 5,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Ref 'static str`),
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
                        FluffyTermExpectationEntry {
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
                                idx: 6,
                                src: ExpectationSource {
                                    expr_idx: 6,
                                    kind: Expr,
                                },
                                expectee: FluffyTerm {
                                    place: None,
                                    base: FluffyTermBase::Ethereal(
                                        EtherealTerm(`Ref 'static str`),
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
                        FluffyTermExpectationEntry {
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
                                        EtherealTerm(`Vec Ref 'static str`),
                                    ),
                                },
                                resolve_progress: ExpectationProgress::Intact,
                            },
                        },
                        FluffyTermExpectationEntry {
                            expectation: Expectation::AnyDerived(
                                ExpectAnyDerived,
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
                                        EtherealTerm(`unit`),
                                    ),
                                },
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