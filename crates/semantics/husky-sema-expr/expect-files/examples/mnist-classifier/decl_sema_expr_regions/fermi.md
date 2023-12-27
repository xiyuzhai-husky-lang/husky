[
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Type(
                                    TypeSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 1,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Prefix {
                            opr: Tilde,
                            opr_regional_token_idx: RegionalTokenIdx(
                                10,
                            ),
                            opd: 1,
                        },
                        SynExprData::List {
                            lbox_regional_token_idx: RegionalTokenIdx(
                                7,
                            ),
                            items: [],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                8,
                            ),
                        },
                        SynExprData::Prefix {
                            opr: Option,
                            opr_regional_token_idx: RegionalTokenIdx(
                                9,
                            ),
                            opd: 2,
                        },
                        SynExprData::ExplicitApplication {
                            function_expr_idx: 3,
                            argument_expr_idx: 4,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::List {
                            lbox_regional_token_idx: RegionalTokenIdx(
                                15,
                            ),
                            items: [],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                16,
                            ),
                        },
                        SynExprData::Prefix {
                            opr: Tilde,
                            opr_regional_token_idx: RegionalTokenIdx(
                                17,
                            ),
                            opd: 6,
                        },
                        SynExprData::ExplicitApplication {
                            function_expr_idx: 7,
                            argument_expr_idx: 8,
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `ConcaveComponent`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `ConcaveComponent`,
                                    regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Pure,
                                access_start: RegionalTokenIdx(
                                    12,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::FieldVariable {
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                    },
                                },
                            },
                            CurrentSynSymbol {
                                modifier: Pure,
                                access_start: RegionalTokenIdx(
                                    19,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::FieldVariable {
                                    ident_token: IdentRegionalToken {
                                        ident: `others`,
                                        regional_token_idx: RegionalTokenIdx(
                                            13,
                                        ),
                                    },
                                },
                            },
                        ],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            FieldVariable {
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 237,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                                ty_expr_idx: 5,
                            },
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                        (
                            FieldVariable {
                                ident_token: IdentRegionalToken {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 425,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                },
                                ty_expr_idx: 9,
                            },
                            ArenaIdxRange(
                                2..3,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::PropsStructFieldType {
                            ident_token: IdentRegionalToken {
                                ident: `matches`,
                                regional_token_idx: RegionalTokenIdx(
                                    5,
                                ),
                            },
                        },
                        syn_expr_idx: 5,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::PropsStructFieldType {
                            ident_token: IdentRegionalToken {
                                ident: `others`,
                                regional_token_idx: RegionalTokenIdx(
                                    13,
                                ),
                            },
                        },
                        syn_expr_idx: 9,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Type(
                        TypeSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 279,
                                },
                            ),
                        ),
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
                                        7,
                                    ),
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    1,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: MajorItem(
                                        Type(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 285,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    2,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Prefix {
                                    opr: LeashType,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    3,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Prefix {
                                    opr: Option,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    4,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        4,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    5,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                VecFunctor {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    6,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: MajorItem(
                                        Type(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 285,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    7,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Prefix {
                                    opr: LeashType,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        7,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    8,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        6,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    9,
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
            sema_expr_roots: [
                (
                    5,
                    (
                        SemaExprIdx(
                            5,
                        ),
                        PropsStructFieldType {
                            ident_token: IdentRegionalToken {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 237,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                ),
                (
                    9,
                    (
                        SemaExprIdx(
                            9,
                        ),
                        PropsStructFieldType {
                            ident_token: IdentRegionalToken {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 425,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    13,
                                ),
                            },
                        },
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
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
                                            ItemPathId(
                                                Id {
                                                    value: 82,
                                                },
                                            ),
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
                                            ItemPathId(
                                                Id {
                                                    value: 285,
                                                },
                                            ),
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
                                            value: 62,
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
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 72,
                                        },
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
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 73,
                                        },
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
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 82,
                                                },
                                            ),
                                        ),
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
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 285,
                                                },
                                            ),
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
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 62,
                                        },
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
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 74,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 73,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 74,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        None,
                        None,
                    ],
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
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: Sort,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
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
                                                    variant: Curry {
                                                        variance: Covariant,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
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
                                                        return_ty: FluffyTerm {
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
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: CurryDestination(
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
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
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
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: CurryDestination(
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
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
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
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: Sort,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
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
                                                    variant: Curry {
                                                        variance: Covariant,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
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
                                                        return_ty: FluffyTerm {
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
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: CurryDestination(
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
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: CurryDestination(
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
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
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
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                EntityPath(
                    TypeOntology(
                        TypePath(
                            ItemPathId(
                                Id {
                                    value: 290,
                                },
                            ),
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        SynExprData::List {
                            lbox_regional_token_idx: RegionalTokenIdx(
                                8,
                            ),
                            items: [],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                9,
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 1,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::ExplicitApplication {
                            function_expr_idx: 1,
                            argument_expr_idx: 2,
                        },
                        SynExprData::Prefix {
                            opr: Tilde,
                            opr_regional_token_idx: RegionalTokenIdx(
                                7,
                            ),
                            opd: 3,
                        },
                        SynExprData::List {
                            lbox_regional_token_idx: RegionalTokenIdx(
                                14,
                            ),
                            items: [],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                15,
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Prefix {
                            opr: Tilde,
                            opr_regional_token_idx: RegionalTokenIdx(
                                19,
                            ),
                            opd: 6,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 3,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Prefix {
                            opr: Option,
                            opr_regional_token_idx: RegionalTokenIdx(
                                23,
                            ),
                            opd: 8,
                        },
                        SynExprData::Ritchie {
                            ritchie_kind: Type(
                                Fn,
                            ),
                            ritchie_kind_regional_token_idx: RegionalTokenIdx(
                                17,
                            ),
                            lpar_token: LparRegionalToken(
                                RegionalTokenIdx(
                                    18,
                                ),
                            ),
                            parameter_ty_items: [
                                SynCommaListItem {
                                    syn_expr_idx: 7,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                21,
                            ),
                            light_arrow_token: Some(
                                LightArrowRegionalToken(
                                    RegionalTokenIdx(
                                        22,
                                    ),
                                ),
                            ),
                            return_ty_syn_expr_idx: Some(
                                9,
                            ),
                        },
                        SynExprData::FunctionApplicationOrCall {
                            function: 5,
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                16,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 10,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                25,
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 4,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `ConcaveComponent`,
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `ConcaveComponent`,
                                    regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `f32`,
                                    regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `FermiMatchResult`,
                                    regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [
                            SynPatternExprData::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `concave_components`,
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                            },
                            SynPatternExprData::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `templates`,
                                    regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                },
                            },
                        ],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [
                            Pure,
                            Pure,
                        ],
                    },
                    pattern_symbol_arena: Arena {
                        data: [
                            SynPatternSymbol::Atom(
                                1,
                            ),
                            SynPatternSymbol::Atom(
                                2,
                            ),
                        ],
                    },
                    pattern_symbol_maps: [
                        [
                            (
                                `concave_components`,
                                1,
                            ),
                        ],
                        [
                            (
                                `templates`,
                                2,
                            ),
                        ],
                    ],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [
                            Pure,
                            Pure,
                        ],
                    },
                },
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [
                            CurrentSynSymbol {
                                modifier: Pure,
                                access_start: RegionalTokenIdx(
                                    6,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                    ident: `concave_components`,
                                    pattern_symbol_idx: 1,
                                },
                            },
                            CurrentSynSymbol {
                                modifier: Pure,
                                access_start: RegionalTokenIdx(
                                    13,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                    ident: `templates`,
                                    pattern_symbol_idx: 2,
                                },
                            },
                        ],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            OrdinaryParenateParameter {
                                syn_pattern_root: ParenateSynPatternExprRoot {
                                    syn_pattern_expr_idx: 1,
                                },
                                ty_expr_idx: 4,
                            },
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                        (
                            OrdinaryParenateParameter {
                                syn_pattern_root: ParenateSynPatternExprRoot {
                                    syn_pattern_expr_idx: 2,
                                },
                                ty_expr_idx: 11,
                            },
                            ArenaIdxRange(
                                2..3,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [
                    SynPatternExprRoot {
                        kind: SynPatternExprRootKind::Parenate,
                        syn_pattern_expr_idx: 1,
                    },
                    SynPatternExprRoot {
                        kind: SynPatternExprRootKind::Parenate,
                        syn_pattern_expr_idx: 2,
                    },
                ],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::ExplicitParameterType,
                        syn_expr_idx: 4,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::ExplicitParameterType,
                        syn_expr_idx: 11,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::ReturnType,
                        syn_expr_idx: 12,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [
                    (
                        1,
                        1,
                    ),
                    (
                        2,
                        2,
                    ),
                ],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 280,
                                },
                            ),
                        ),
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
                                        8,
                                    ),
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    1,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: MajorItem(
                                        Type(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 285,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    2,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    3,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Prefix {
                                    opr: LeashType,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        3,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    4,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                VecFunctor {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    5,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: MajorItem(
                                        Type(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 285,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    6,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Prefix {
                                    opr: LeashType,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        6,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    7,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    path: MajorItem(
                                        Type(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    8,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Prefix {
                                    opr: Option,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    9,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Ritchie {
                                    ritchie_kind_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    ritchie_kind: Type(
                                        Fn,
                                    ),
                                    lpar_token: LparRegionalToken(
                                        RegionalTokenIdx(
                                            18,
                                        ),
                                    ),
                                    parameter_ty_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                7,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    light_arrow_token: Some(
                                        LightArrowRegionalToken(
                                            RegionalTokenIdx(
                                                22,
                                            ),
                                        ),
                                    ),
                                    return_ty_sema_expr_idx: Some(
                                        SemaExprIdx(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    10,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FunctionApplication {
                                    function_sema_expr_idx: SemaExprIdx(
                                        5,
                                    ),
                                    argument_sema_expr_idx: SemaExprIdx(
                                        10,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    11,
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
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    path: MajorItem(
                                        Type(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 290,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    12,
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
            sema_expr_roots: [
                (
                    4,
                    (
                        SemaExprIdx(
                            4,
                        ),
                        ExplicitParameterType,
                    ),
                ),
                (
                    11,
                    (
                        SemaExprIdx(
                            11,
                        ),
                        ExplicitParameterType,
                    ),
                ),
                (
                    12,
                    (
                        SemaExprIdx(
                            12,
                        ),
                        ReturnType,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [
                    None,
                    None,
                ],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
                    None,
                    None,
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
                                            ItemPathId(
                                                Id {
                                                    value: 82,
                                                },
                                            ),
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
                                            ItemPathId(
                                                Id {
                                                    value: 285,
                                                },
                                            ),
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
                                            value: 75,
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
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 76,
                                        },
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
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 82,
                                                },
                                            ),
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
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 285,
                                                },
                                            ),
                                        ),
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
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 62,
                                        },
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
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
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
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 53,
                                        },
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
                                Ritchie(
                                    EtherealTermRitchie(
                                        Id {
                                            value: 2,
                                        },
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
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 77,
                                        },
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
                                EntityPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 290,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 76,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        StackPure {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 77,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        None,
                        None,
                    ],
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
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: Sort,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
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
                                                    variant: Curry {
                                                        variance: Covariant,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
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
                                                        return_ty: FluffyTerm {
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
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: CurryDestination(
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
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: CurryDestination(
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
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
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
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: Sort,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FluffyTerm {
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
                                                    variant: Curry {
                                                        variance: Covariant,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
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
                                                        return_ty: FluffyTerm {
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
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: CurryDestination(
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
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
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
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
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
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsExactly(
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
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Ok(
                                            Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Const,
                                        ty_expected: FluffyTerm {
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
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
                                    resolve_progress: Resolved(
                                        Err(
                                            Original(
                                                Place(
                                                    CannotConvertToConst,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
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
                            },
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
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
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TypeImplBlock(
                    TypeImplBlockSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::ImplBlock(
                                ImplBlockSynNodePathData::TypeImplBlock(
                                    TypeImplBlockSynNodePathData {
                                        path: TypeImplBlockPath(
                                            ItemPathId(
                                                Id {
                                                    value: 405,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: None,
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::ImplBlock(
                        ImplBlockSynNodePath::TypeImplBlock(
                            TypeImplBlockSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::ImplBlock(
                                        ImplBlockSynNodePathData::TypeImplBlock(
                                            TypeImplBlockSynNodePathData {
                                                path: TypeImplBlockPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 405,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 1,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `FermiMatchResult`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::SelfType,
                        syn_expr_idx: 1,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 377,
                                },
                            ),
                        ),
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
                                                ItemPathId(
                                                    Id {
                                                        value: 290,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    1,
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
            sema_expr_roots: [
                (
                    1,
                    (
                        SemaExprIdx(
                            1,
                        ),
                        SelfType,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
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
                                            ItemPathId(
                                                Id {
                                                    value: 290,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
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
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
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
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                EntityPath(
                    TypeOntology(
                        TypePath(
                            ItemPathId(
                                Id {
                                    value: 290,
                                },
                            ),
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: Some(
                    SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: SynNodeRegionPath::Decl(
                                ItemSynNodePath::ImplBlock(
                                    ImplBlockSynNodePath::TypeImplBlock(
                                        TypeImplBlockSynNodePath(
                                            ItemSynNodePathId {
                                                data: ItemSynNodePathData::ImplBlock(
                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                        TypeImplBlockSynNodePathData {
                                                            path: TypeImplBlockPath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 405,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            syn_expr_arena: Arena {
                                data: [
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `FermiMatchResult`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    2,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: SynPatternExprRegion {
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_contracts: ArenaMap {
                                    data: [],
                                },
                                pattern_symbol_arena: Arena {
                                    data: [],
                                },
                                pattern_symbol_maps: [],
                                pattern_symbol_modifiers: ArenaMap {
                                    data: [],
                                },
                            },
                            symbol_region: SynSymbolRegionData {
                                inherited_syn_symbol_arena: Arena {
                                    data: [],
                                },
                                current_syn_symbol_arena: Arena {
                                    data: [],
                                },
                                allow_self_type: True,
                                allow_self_value: False,
                                pattern_ty_constraints: [],
                            },
                            syn_pattern_expr_roots: [],
                            syn_expr_roots: [
                                SynExprRoot {
                                    kind: SynExprRootKind::SelfType,
                                    syn_expr_idx: 1,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                            syn_pattern_to_current_syn_symbol_map: [],
                        },
                    },
                ),
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::AssociatedItem(
                        AssociatedItemSynNodePath::TypeItem(
                            TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssociatedItem(
                                        AssociatedItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 1,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `f32`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: True,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::ReturnType,
                        syn_expr_idx: 1,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 399,
                                },
                            ),
                        ),
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
                                                ItemPathId(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    1,
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
            sema_expr_roots: [
                (
                    1,
                    (
                        SemaExprIdx(
                            1,
                        ),
                        ReturnType,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
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
                                            ItemPathId(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
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
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
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
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                EntityPath(
                    TypeOntology(
                        TypePath(
                            ItemPathId(
                                Id {
                                    value: 290,
                                },
                            ),
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`, `MemoizedField`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: Some(
                    SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: SynNodeRegionPath::Decl(
                                ItemSynNodePath::ImplBlock(
                                    ImplBlockSynNodePath::TypeImplBlock(
                                        TypeImplBlockSynNodePath(
                                            ItemSynNodePathId {
                                                data: ItemSynNodePathData::ImplBlock(
                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                        TypeImplBlockSynNodePathData {
                                                            path: TypeImplBlockPath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 405,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            syn_expr_arena: Arena {
                                data: [
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `FermiMatchResult`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    2,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: SynPatternExprRegion {
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_contracts: ArenaMap {
                                    data: [],
                                },
                                pattern_symbol_arena: Arena {
                                    data: [],
                                },
                                pattern_symbol_maps: [],
                                pattern_symbol_modifiers: ArenaMap {
                                    data: [],
                                },
                            },
                            symbol_region: SynSymbolRegionData {
                                inherited_syn_symbol_arena: Arena {
                                    data: [],
                                },
                                current_syn_symbol_arena: Arena {
                                    data: [],
                                },
                                allow_self_type: True,
                                allow_self_value: False,
                                pattern_ty_constraints: [],
                            },
                            syn_pattern_expr_roots: [],
                            syn_expr_roots: [
                                SynExprRoot {
                                    kind: SynExprRootKind::SelfType,
                                    syn_expr_idx: 1,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                            syn_pattern_to_current_syn_symbol_map: [],
                        },
                    },
                ),
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::AssociatedItem(
                        AssociatedItemSynNodePath::TypeItem(
                            TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssociatedItem(
                                        AssociatedItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`, `MemoizedField`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 1,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `f32`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: True,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::ReturnType,
                        syn_expr_idx: 1,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 400,
                                },
                            ),
                        ),
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
                                                ItemPathId(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    1,
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
            sema_expr_roots: [
                (
                    1,
                    (
                        SemaExprIdx(
                            1,
                        ),
                        ReturnType,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
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
                                            ItemPathId(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
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
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
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
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                EntityPath(
                    TypeOntology(
                        TypePath(
                            ItemPathId(
                                Id {
                                    value: 290,
                                },
                            ),
                        ),
                    ),
                ),
            ),
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Decl(
            ItemSynNodePath::AssociatedItem(
                AssociatedItemSynNodePath::TypeItem(
                    TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssociatedItem(
                                AssociatedItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`, `MemoizedField`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
        syn_expr_region: SynExprRegion {
            data: SynExprRegionData {
                parent: Some(
                    SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: SynNodeRegionPath::Decl(
                                ItemSynNodePath::ImplBlock(
                                    ImplBlockSynNodePath::TypeImplBlock(
                                        TypeImplBlockSynNodePath(
                                            ItemSynNodePathId {
                                                data: ItemSynNodePathData::ImplBlock(
                                                    ImplBlockSynNodePathData::TypeImplBlock(
                                                        TypeImplBlockSynNodePathData {
                                                            path: TypeImplBlockPath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 405,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            syn_expr_arena: Arena {
                                data: [
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `FermiMatchResult`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    2,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: SynPatternExprRegion {
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_contracts: ArenaMap {
                                    data: [],
                                },
                                pattern_symbol_arena: Arena {
                                    data: [],
                                },
                                pattern_symbol_maps: [],
                                pattern_symbol_modifiers: ArenaMap {
                                    data: [],
                                },
                            },
                            symbol_region: SynSymbolRegionData {
                                inherited_syn_symbol_arena: Arena {
                                    data: [],
                                },
                                current_syn_symbol_arena: Arena {
                                    data: [],
                                },
                                allow_self_type: True,
                                allow_self_value: False,
                                pattern_ty_constraints: [],
                            },
                            syn_pattern_expr_roots: [],
                            syn_expr_roots: [
                                SynExprRoot {
                                    kind: SynExprRootKind::SelfType,
                                    syn_expr_idx: 1,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                            syn_pattern_to_current_syn_symbol_map: [],
                        },
                    },
                ),
                path: SynNodeRegionPath::Decl(
                    ItemSynNodePath::AssociatedItem(
                        AssociatedItemSynNodePath::TypeItem(
                            TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssociatedItem(
                                        AssociatedItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`, `MemoizedField`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 1,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`core::num::f32`, `Extern`),
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `f32`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: [],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: True,
                    allow_self_value: True,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::ReturnType,
                        syn_expr_idx: 1,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Decl(
                AssociatedItem(
                    TypeItem(
                        TypeItemSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 401,
                                },
                            ),
                        ),
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
                                                ItemPathId(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: OntologyConstructor,
                                    instantiation: None,
                                },
                            ),
                            immediate_ty_result: Ok(
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
                            expectation_idx_and_ty: Some(
                                (
                                    1,
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
            sema_expr_roots: [
                (
                    1,
                    (
                        SemaExprIdx(
                            1,
                        ),
                        ReturnType,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
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
                                            ItemPathId(
                                                Id {
                                                    value: 53,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [],
                },
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
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
                            FluffyTermExpectationEntry {
                                expectation: EqsSort(
                                    ExpectEqsCategory {
                                        smallest_universe: TermUniverse(
                                            1,
                                        ),
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    implicit_parameter_substitutions: [],
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
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: Some(
                EntityPath(
                    TypeOntology(
                        TypePath(
                            ItemPathId(
                                Id {
                                    value: 290,
                                },
                            ),
                        ),
                    ),
                ),
            ),
        },
    },
]