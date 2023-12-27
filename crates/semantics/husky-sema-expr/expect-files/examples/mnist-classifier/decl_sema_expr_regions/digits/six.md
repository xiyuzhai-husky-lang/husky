[
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
                                            path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                        4,
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
                    allow_self_type: False,
                    allow_self_value: False,
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
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 299,
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
            self_ty: None,
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
                                            path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                        4,
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
                    allow_self_type: False,
                    allow_self_value: False,
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
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 300,
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
            self_ty: None,
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
                                            path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                                    path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::ExplicitApplication {
                            function_expr_idx: 1,
                            argument_expr_idx: 2,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 4,
                            opt_path: Some(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    ident: `Six`,
                                                    index: U8(
                                                        6,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                        SynExprData::ExplicitApplication {
                            function_expr_idx: 3,
                            argument_expr_idx: 4,
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `OneVsAll`,
                                    regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `MnistLabel`,
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `MnistLabel`,
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Subitem {
                            parent: 3,
                            colon_colon_token: ColonColonRegionalToken(
                                RegionalTokenIdx(
                                    11,
                                ),
                            ),
                            ident_token: Ok(
                                IdentRegionalToken {
                                    ident: `Six`,
                                    regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                },
                            ),
                            path: Ok(
                                PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath(
                                        ItemPathId {
                                            data: ItemPathData::TypeVariant(
                                                TypeVariantPathData {
                                                    parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    ident: `Six`,
                                                    index: U8(
                                                        6,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
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
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::ReturnType,
                        syn_expr_idx: 5,
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
                    Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 301,
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
                                                        value: 357,
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
                            expectation_idx_and_ty: Some(
                                (
                                    1,
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
                                                        value: 368,
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
                                        Curry(
                                            EtherealTermCurry(
                                                Id {
                                                    value: 14,
                                                },
                                            ),
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
                                            Curry(
                                                EtherealTermCurry(
                                                    Id {
                                                        value: 14,
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
                                    path_expr_idx: 4,
                                    path: TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 375,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FluffyInstantiation {
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 368,
                                                        },
                                                    ),
                                                ),
                                            ),
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
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 368,
                                                            },
                                                        ),
                                                    ),
                                                ),
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
                                        3,
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
                                                    value: 368,
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
                                                    value: 357,
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
                                            value: 60,
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
                                EntityPath(
                                    TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 375,
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
                        5,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Application(
                                    EtherealTermApplication(
                                        Id {
                                            value: 68,
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
                                                        value: 13,
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
                                                            Curry(
                                                                EtherealTermCurry(
                                                                    Id {
                                                                        value: 12,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    variant: Curry {
                                                        variance: Independent,
                                                        parameter_rune: Some(
                                                            FluffyTermRune(
                                                                FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        Rune(
                                                                            EtherealTermRune(
                                                                                Id {
                                                                                    value: 1,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
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
                                                                Curry(
                                                                    EtherealTermCurry(
                                                                        Id {
                                                                            value: 12,
                                                                        },
                                                                    ),
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
                                expectation: EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: Sort,
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
                                            Curry(
                                                EtherealTermCurry(
                                                    Id {
                                                        value: 14,
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
                                                        variance: Independent,
                                                        parameter_rune: None,
                                                        parameter_ty: FluffyTerm {
                                                            place: None,
                                                            base: Ethereal(
                                                                EntityPath(
                                                                    TypeOntology(
                                                                        TypePath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 368,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
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
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 368,
                                                                },
                                                            ),
                                                        ),
                                                    ),
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
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 368,
                                                            },
                                                        ),
                                                    ),
                                                ),
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
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
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
                                                    path: FugitivePath(`mnist_classifier::digits::six::upmost`, `FunctionFn`),
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
                                6,
                            ),
                            opd: 1,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
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
                                10,
                            ),
                            opd: 3,
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
                                        7,
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
                                        11,
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
                        data: [
                            SynPatternExprData::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                },
                            },
                        ],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [
                            Pure,
                        ],
                    },
                    pattern_symbol_arena: Arena {
                        data: [
                            SynPatternSymbol::Atom(
                                1,
                            ),
                        ],
                    },
                    pattern_symbol_maps: [
                        [
                            (
                                `cc`,
                                1,
                            ),
                        ],
                    ],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [
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
                                    5,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                    ident: `cc`,
                                    pattern_symbol_idx: 1,
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
                                ty_expr_idx: 2,
                            },
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [
                    SynPatternExprRoot {
                        kind: SynPatternExprRootKind::Parenate,
                        syn_pattern_expr_idx: 1,
                    },
                ],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::ExplicitParameterType,
                        syn_expr_idx: 2,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::ReturnType,
                        syn_expr_idx: 4,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [
                    (
                        1,
                        1,
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
                                    value: 302,
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
                        SemaExprEntry {
                            data_result: Ok(
                                Prefix {
                                    opr: LeashType,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        1,
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
                                PrincipalEntityPath {
                                    path_expr_idx: 2,
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
                                        10,
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
                    2,
                    (
                        SemaExprIdx(
                            2,
                        ),
                        ExplicitParameterType,
                    ),
                ),
                (
                    4,
                    (
                        SemaExprIdx(
                            4,
                        ),
                        ReturnType,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [
                    None,
                ],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
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
                        2,
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
                        3,
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
                        4,
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
                                                    value: 62,
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
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
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
                                                    path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `FunctionFn`),
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
                                6,
                            ),
                            opd: 1,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
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
                                10,
                            ),
                            opd: 3,
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
                                        7,
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
                                        11,
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
                        data: [
                            SynPatternExprData::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                },
                            },
                        ],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [
                            Pure,
                        ],
                    },
                    pattern_symbol_arena: Arena {
                        data: [
                            SynPatternSymbol::Atom(
                                1,
                            ),
                        ],
                    },
                    pattern_symbol_maps: [
                        [
                            (
                                `cc`,
                                1,
                            ),
                        ],
                    ],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [
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
                                    5,
                                ),
                                access_end: None,
                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                    ident: `cc`,
                                    pattern_symbol_idx: 1,
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
                                ty_expr_idx: 2,
                            },
                            ArenaIdxRange(
                                1..2,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [
                    SynPatternExprRoot {
                        kind: SynPatternExprRootKind::Parenate,
                        syn_pattern_expr_idx: 1,
                    },
                ],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::ExplicitParameterType,
                        syn_expr_idx: 2,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::ReturnType,
                        syn_expr_idx: 4,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [
                    (
                        1,
                        1,
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
                                    value: 303,
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
                        SemaExprEntry {
                            data_result: Ok(
                                Prefix {
                                    opr: LeashType,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                    opd_sema_expr_idx: SemaExprIdx(
                                        1,
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
                                PrincipalEntityPath {
                                    path_expr_idx: 2,
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
                                        10,
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
                    2,
                    (
                        SemaExprIdx(
                            2,
                        ),
                        ExplicitParameterType,
                    ),
                ),
                (
                    4,
                    (
                        SemaExprIdx(
                            4,
                        ),
                        ReturnType,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [
                    None,
                ],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
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
                        2,
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
                        3,
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
                        4,
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
                                                    value: 62,
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
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: None,
            self_ty: None,
        },
    },
]