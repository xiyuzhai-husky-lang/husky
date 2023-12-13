[
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
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
                                ItemSynNodePath::MajorItem(
                                    MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath(
                                            ItemSynNodePathId {
                                                data: ItemSynNodePathData::MajorItem(
                                                    MajorItemSynNodePathData::Fugitive(
                                                        FugitiveSynNodePathData {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
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
                                            5,
                                        ),
                                        items: [],
                                        rbox_regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 1,
                                        argument_expr_idx: 2,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `ConnectedComponent`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                    syn_expr_idx: 3,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                            syn_pattern_to_current_syn_symbol_map: [],
                        },
                    },
                ),
                path: SynNodeRegionPath::Defn(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
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
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist::input`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::FunctionApplicationOrCall {
                            function: 1,
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 2,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                4,
                            ),
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `find_connected_components`,
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `input`,
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist::input`, `Val`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Eval {
                            expr_idx: 3,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                    ],
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
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 3,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::BlockExpr,
                        syn_expr_idx: 4,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 307,
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
                                        Fugitive(
                                            FugitivePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 223,
                                                    },
                                                ),
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
                                        Ritchie(
                                            EtherealTermRitchie(
                                                Id {
                                                    value: 38,
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
                                        Fugitive(
                                            FugitivePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 352,
                                                    },
                                                ),
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
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 112,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FunctionFnCall {
                                    function_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        Regular(
                                            FluffyRitchieRegularParameter {
                                                contract: Pure,
                                                ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 350,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaRegularCallListItem {
                                                argument_expr_idx: SemaExprIdx(
                                                    2,
                                                ),
                                                coersion: Some(
                                                    Deref(
                                                        Leash,
                                                    ),
                                                ),
                                                separator: None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        4,
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
                                                    value: 50,
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
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 50,
                                                },
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
                                        3,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            Trivial(
                                                TrivialFluffyCoersion {
                                                    expectee_place: Transient,
                                                },
                                            ),
                                        ),
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
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 50,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    4,
                    (
                        SemaExprIdx(
                            4,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
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
                                        final_destination: TypeOntology,
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Ritchie(
                                                EtherealTermRitchie(
                                                    Id {
                                                        value: 38,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    template_parameter_substitutions: [],
                                                    return_ty: FluffyTerm {
                                                        place: None,
                                                        base: Ethereal(
                                                            Application(
                                                                EtherealTermApplication(
                                                                    Id {
                                                                        value: 50,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    variant: Ritchie {
                                                        ritchie_kind: Type(
                                                            Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            Regular(
                                                                FluffyRitchieRegularParameter {
                                                                    contract: Pure,
                                                                    ty: FluffyTerm {
                                                                        place: None,
                                                                        base: Ethereal(
                                                                            EntityPath(
                                                                                TypeOntology(
                                                                                    TypePath(
                                                                                        ItemPathId(
                                                                                            Id {
                                                                                                value: 350,
                                                                                            },
                                                                                        ),
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
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 350,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 112,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Deref(
                                                    Leash,
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 50,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 50,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 50,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 50,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
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
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 50,
                        },
                    ),
                ),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                ItemSynNodePath::MajorItem(
                                    MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath(
                                            ItemSynNodePathId {
                                                data: ItemSynNodePathData::MajorItem(
                                                    MajorItemSynNodePathData::Fugitive(
                                                        FugitiveSynNodePathData {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::Prefix {
                                        opr: Tilde,
                                        opr_regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                        opd: 1,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `ConnectedComponent`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                    syn_expr_idx: 2,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                            syn_pattern_to_current_syn_symbol_map: [],
                        },
                    },
                ),
                path: SynNodeRegionPath::Defn(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                5,
                            ),
                            LiteralData::Integer(
                                UnspecifiedRegular(
                                    0,
                                ),
                            ),
                        ),
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                10,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 127,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 1,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::FrameVarDecl {
                            regional_token_idx: RegionalTokenIdx(
                                12,
                            ),
                            ident: `i`,
                            frame_var_symbol_idx: 3,
                            current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                4,
                            ),
                        },
                        SynExprData::MethodApplicationOrCall {
                            self_argument: 3,
                            dot_regional_token_idx: RegionalTokenIdx(
                                15,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `ilen`,
                                regional_token_idx: RegionalTokenIdx(
                                    16,
                                ),
                            },
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                17,
                            ),
                            items: [],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                18,
                            ),
                        },
                        SynExprData::Binary {
                            lopd: 4,
                            opr: Comparison(
                                Less,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                13,
                            ),
                            ropd: 5,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::CurrentSynSymbol {
                            ident: `i`,
                            regional_token_idx: RegionalTokenIdx(
                                25,
                            ),
                            current_syn_symbol_idx: 3,
                            current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                4,
                            ),
                        },
                        SynExprData::IndexOrCompositionWithList {
                            owner: 7,
                            lbox_regional_token_idx: RegionalTokenIdx(
                                24,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 8,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                26,
                            ),
                        },
                        SynExprData::Field {
                            owner: 9,
                            dot_regional_token_idx: RegionalTokenIdx(
                                27,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `row_span_sum`,
                                regional_token_idx: RegionalTokenIdx(
                                    28,
                                ),
                            },
                        },
                        SynExprData::CurrentSynSymbol {
                            ident: `row_span_sum`,
                            regional_token_idx: RegionalTokenIdx(
                                30,
                            ),
                            current_syn_symbol_idx: 4,
                            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                pattern_symbol_idx: 3,
                            },
                        },
                        SynExprData::CurrentSynSymbol {
                            ident: `max_row_span_sum`,
                            regional_token_idx: RegionalTokenIdx(
                                32,
                            ),
                            current_syn_symbol_idx: 2,
                            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                pattern_symbol_idx: 2,
                            },
                        },
                        SynExprData::Binary {
                            lopd: 11,
                            opr: Comparison(
                                Greater,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                31,
                            ),
                            ropd: 12,
                        },
                        SynExprData::CurrentSynSymbol {
                            ident: `max_row_span_sum`,
                            regional_token_idx: RegionalTokenIdx(
                                34,
                            ),
                            current_syn_symbol_idx: 2,
                            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                pattern_symbol_idx: 2,
                            },
                        },
                        SynExprData::CurrentSynSymbol {
                            ident: `row_span_sum`,
                            regional_token_idx: RegionalTokenIdx(
                                36,
                            ),
                            current_syn_symbol_idx: 4,
                            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                pattern_symbol_idx: 3,
                            },
                        },
                        SynExprData::Binary {
                            lopd: 14,
                            opr: Assign,
                            opr_regional_token_idx: RegionalTokenIdx(
                                35,
                            ),
                            ropd: 15,
                        },
                        SynExprData::CurrentSynSymbol {
                            ident: `i0`,
                            regional_token_idx: RegionalTokenIdx(
                                37,
                            ),
                            current_syn_symbol_idx: 1,
                            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                pattern_symbol_idx: 1,
                            },
                        },
                        SynExprData::CurrentSynSymbol {
                            ident: `i`,
                            regional_token_idx: RegionalTokenIdx(
                                39,
                            ),
                            current_syn_symbol_idx: 3,
                            current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                4,
                            ),
                        },
                        SynExprData::Binary {
                            lopd: 17,
                            opr: Assign,
                            opr_regional_token_idx: RegionalTokenIdx(
                                38,
                            ),
                            ropd: 18,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 3,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::CurrentSynSymbol {
                            ident: `i0`,
                            regional_token_idx: RegionalTokenIdx(
                                43,
                            ),
                            current_syn_symbol_idx: 1,
                            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                pattern_symbol_idx: 1,
                            },
                        },
                        SynExprData::IndexOrCompositionWithList {
                            owner: 20,
                            lbox_regional_token_idx: RegionalTokenIdx(
                                42,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 21,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                44,
                            ),
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                5..9,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `connected_components`,
                                    regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `connected_components`,
                                    regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `connected_components`,
                                    regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Eval {
                            expr_idx: 16,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                        SynStmtData::Eval {
                            expr_idx: 19,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    20,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_expr_root: LetSynPatternExprRoot {
                                        syn_pattern_expr_idx: 3,
                                    },
                                    variables: ArenaIdxRange(
                                        4..5,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        22,
                                    ),
                                ),
                            ),
                            initial_value: 10,
                        },
                        SynStmtData::IfElse {
                            if_branch: SynIfBranch {
                                if_token: IfRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                },
                                condition: Ok(
                                    13,
                                ),
                                eol_colon: Ok(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            33,
                                        ),
                                    },
                                ),
                                stmts: ArenaIdxRange(
                                    1..3,
                                ),
                            },
                            elif_branches: [],
                            else_branch: None,
                        },
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    1,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_expr_root: LetSynPatternExprRoot {
                                        syn_pattern_expr_idx: 1,
                                    },
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                            ),
                            initial_value: 1,
                        },
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    6,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_expr_root: LetSynPatternExprRoot {
                                        syn_pattern_expr_idx: 2,
                                    },
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        9,
                                    ),
                                ),
                            ),
                            initial_value: 2,
                        },
                        SynStmtData::ForBetween {
                            for_token: StmtForRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    11,
                                ),
                            },
                            particulars: SynForBetweenParticulars {
                                for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                    12,
                                ),
                                for_between_loop_var_ident: `i`,
                                for_between_loop_var_expr_idx: 4,
                                range: Ok(
                                    SynForBetweenRange {
                                        initial_boundary: SynForBetweenLoopBoundary {
                                            bound_expr: None,
                                            kind: LowerClosed,
                                        },
                                        final_boundary: SynForBetweenLoopBoundary {
                                            bound_expr: Some(
                                                5,
                                            ),
                                            kind: UpperOpen,
                                        },
                                        step: Constant(
                                            1,
                                        ),
                                    },
                                ),
                            },
                            for_loop_var_symbol_idx: 3,
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            19,
                                        ),
                                    },
                                ),
                            ),
                            block: ArenaIdxRange(
                                3..5,
                            ),
                        },
                        SynStmtData::Return {
                            return_token: ReturnRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    40,
                                ),
                            },
                            result: 22,
                        },
                    ],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: Some(
                                    Mut(
                                        MutRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                2,
                                            ),
                                        },
                                    ),
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: `i0`,
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                            },
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: Some(
                                    Mut(
                                        MutRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                7,
                                            ),
                                        },
                                    ),
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: `max_row_span_sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            },
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: None,
                                ident_token: IdentRegionalToken {
                                    ident: `row_span_sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                },
                            },
                        ],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [
                            Move,
                            Move,
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
                            SynPatternSymbol::Atom(
                                3,
                            ),
                        ],
                    },
                    pattern_symbol_maps: [
                        [
                            (
                                `i0`,
                                1,
                            ),
                        ],
                        [
                            (
                                `max_row_span_sum`,
                                2,
                            ),
                        ],
                        [
                            (
                                `row_span_sum`,
                                3,
                            ),
                        ],
                    ],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [
                            Mut,
                            Mut,
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
                                modifier: Mut,
                                access_start: RegionalTokenIdx(
                                    4,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            45,
                                        ),
                                    ),
                                ),
                                data: CurrentSynSymbolData::LetVariable {
                                    ident: `i0`,
                                    pattern_symbol_idx: 1,
                                },
                            },
                            CurrentSynSymbol {
                                modifier: Mut,
                                access_start: RegionalTokenIdx(
                                    9,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            45,
                                        ),
                                    ),
                                ),
                                data: CurrentSynSymbolData::LetVariable {
                                    ident: `max_row_span_sum`,
                                    pattern_symbol_idx: 2,
                                },
                            },
                            CurrentSynSymbol {
                                modifier: Pure,
                                access_start: RegionalTokenIdx(
                                    20,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            40,
                                        ),
                                    ),
                                ),
                                data: CurrentSynSymbolData::LoopVariable {
                                    ident: `i`,
                                    expr_idx: 4,
                                },
                            },
                            CurrentSynSymbol {
                                modifier: Pure,
                                access_start: RegionalTokenIdx(
                                    22,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            40,
                                        ),
                                    ),
                                ),
                                data: CurrentSynSymbolData::LetVariable {
                                    ident: `row_span_sum`,
                                    pattern_symbol_idx: 3,
                                },
                            },
                        ],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            LoopVariable,
                            ArenaIdxRange(
                                3..4,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [
                    SynPatternExprRoot {
                        kind: SynPatternExprRootKind::Let,
                        syn_pattern_expr_idx: 1,
                    },
                    SynPatternExprRoot {
                        kind: SynPatternExprRootKind::Let,
                        syn_pattern_expr_idx: 2,
                    },
                    SynPatternExprRoot {
                        kind: SynPatternExprRootKind::Let,
                        syn_pattern_expr_idx: 3,
                    },
                ],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::LetStmtInitialValue,
                        syn_expr_idx: 1,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::LetStmtInitialValue,
                        syn_expr_idx: 2,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::LetStmtInitialValue,
                        syn_expr_idx: 10,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 16,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 19,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::ReturnExpr,
                        syn_expr_idx: 22,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::BlockExpr,
                        syn_expr_idx: 23,
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
                    (
                        3,
                        4,
                    ),
                ],
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 308,
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
                                Literal(
                                    RegionalTokenIdx(
                                        5,
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
                                    place: Some(
                                        Const,
                                    ),
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
                                        10,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 127,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            1,
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
                                                ItemPathId(
                                                    Id {
                                                        value: 318,
                                                    },
                                                ),
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
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 107,
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
                                        3,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 134,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            16,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFn(
                                            MethodFnFluffySignature {
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 191,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: Ethereal(
                                                            Application(
                                                                EtherealTermApplication(
                                                                    Id {
                                                                        value: 50,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 43,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EtherealTermSymbol(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                            Explicit(
                                                                FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 221,
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
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        18,
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 43,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 250,
                                            },
                                        ),
                                    ),
                                    frame_var_symbol_idx: 3,
                                    current_syn_symbol_kind: LoopVariable(
                                        4,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 43,
                                                        },
                                                    ),
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
                                                ItemPathId(
                                                    Id {
                                                        value: 318,
                                                    },
                                                ),
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
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 107,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 250,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: LoopVariable(
                                        4,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 43,
                                                        },
                                                    ),
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
                                        6,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                7,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 221,
                                                                    },
                                                                ),
                                                            ),
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
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 221,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 221,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 255,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            28,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 221,
                                                },
                                            ),
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
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
                                            path: TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 383,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            instantiation: FluffyInstantiation {
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: None,
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
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
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 255,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    4,
                                                ),
                                            ),
                                        },
                                    ),
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
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 533,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            1,
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
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    ropd: SemaExprIdx(
                                        11,
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 21,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 533,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 2,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 255,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    current_syn_symbol_idx: 4,
                                    current_syn_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 3,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    4,
                                                ),
                                            ),
                                        },
                                    ),
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
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        13,
                                    ),
                                    opr: Assign,
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    ropd: SemaExprIdx(
                                        14,
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 331,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
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
                                CurrentSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 250,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    current_syn_symbol_idx: 3,
                                    current_syn_symbol_kind: LoopVariable(
                                        4,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 43,
                                                        },
                                                    ),
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
                                        16,
                                    ),
                                    opr: Assign,
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    ropd: SemaExprIdx(
                                        17,
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
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
                                                ItemPathId(
                                                    Id {
                                                        value: 318,
                                                    },
                                                ),
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
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 107,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 331,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
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
                                Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        19,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                20,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        44,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 221,
                                                                    },
                                                                ),
                                                            ),
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
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 221,
                                                        },
                                                    ),
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
                                            5..9,
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 22,
                                                        },
                                                    ),
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
                                        15,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            Trivial(
                                                TrivialFluffyCoersion {
                                                    expectee_place: Transient,
                                                },
                                            ),
                                        ),
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
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
                                        18,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            Trivial(
                                                TrivialFluffyCoersion {
                                                    expectee_place: Transient,
                                                },
                                            ),
                                        ),
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
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
                                            20,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 3,
                                        },
                                        variables: ArenaIdxRange(
                                            4..5,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            22,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        9,
                                    ),
                                    coersion: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
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
                                    if_branch: SemaIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                29,
                                            ),
                                        },
                                        condition: Other {
                                            sema_expr_idx: SemaExprIdx(
                                                12,
                                            ),
                                            conversion: None,
                                        },
                                        eol_colon_token: EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                33,
                                            ),
                                        },
                                        stmts: SemaStmtIdxRange(
                                            ArenaIdxRange(
                                                1..3,
                                            ),
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
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
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 1,
                                        },
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    coersion: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
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
                                            6,
                                        ),
                                    },
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 2,
                                        },
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            9,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    coersion: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            11,
                                        ),
                                    },
                                    particulars: SemaForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                        for_between_loop_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 250,
                                                },
                                            ),
                                        ),
                                        for_between_loop_var_expr_idx: SemaExprIdx(
                                            5,
                                        ),
                                        range: SemaForBetweenRange {
                                            initial_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: None,
                                                kind: LowerClosed,
                                            },
                                            final_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    SemaExprIdx(
                                                        4,
                                                    ),
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    for_loop_var_symbol_idx: 3,
                                    eol_colon: Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                19,
                                            ),
                                        },
                                    ),
                                    block: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            3..5,
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            40,
                                        ),
                                    },
                                    result: SemaExprIdx(
                                        21,
                                    ),
                                    coersion: Some(
                                        PlaceToLeash,
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 22,
                                                        },
                                                    ),
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
            sema_expr_roots: [
                (
                    23,
                    (
                        SemaExprIdx(
                            22,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
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
                        },
                    ),
                ],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            0,
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
                                    place: Some(
                                        Const,
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            1,
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
                                    place: Some(
                                        Leashed,
                                    ),
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
                                    USize(
                                        TermUSizeLiteral(
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
                        2,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    F32(
                                        TermF32Literal(
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
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    3,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 43,
                                                        },
                                                    ),
                                                ),
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
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    4,
                                                ),
                                            ),
                                        },
                                    ),
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
                        entries: [
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        1,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 52,
                                                                },
                                                            ),
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
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 52,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 52,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            },
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        2,
                                    ),
                                    hole_kind: UnspecifiedFloatType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            ),
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
                                    constraints: [
                                        CoercibleInto {
                                            target: FluffyTerm {
                                                place: Some(
                                                    ImmutableStackOwned {
                                                        location: StackLocationIdx(
                                                            ShiftedU32(
                                                                4,
                                                            ),
                                                        ),
                                                    },
                                                ),
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
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
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
                        ],
                        first_unresolved_term_idx: 2,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 107,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
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
                                                                value: 43,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 43,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 43,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: ImmutableStackOwned {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    3,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 107,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 43,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 221,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
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
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
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
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            4,
                                                        ),
                                                    ),
                                                },
                                            ),
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
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: MutableStackOwned {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    2,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ConditionType(
                                    ExpectConditionType,
                                ),
                                meta: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
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
                                                                value: 21,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Hollow(
                                            HollowTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyDerived(
                                    ExpectAnyDerived,
                                ),
                                meta: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
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
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
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
                                                                value: 23,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyDerived(
                                    ExpectAnyDerived,
                                ),
                                meta: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 43,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 19,
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
                                                                value: 23,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 19,
                                    src: ExpectationSource {
                                        syn_expr_idx: 20,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 107,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 20,
                                    src: ExpectationSource {
                                        syn_expr_idx: 21,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 52,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 21,
                                    src: ExpectationSource {
                                        syn_expr_idx: 22,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: MutableStackOwned {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    1,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 47,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 22,
                                    src: ExpectationSource {
                                        syn_expr_idx: 22,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 221,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                PlaceToLeash,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 47,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 23,
                                    src: ExpectationSource {
                                        syn_expr_idx: 23,
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
                                                                value: 22,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Never,
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
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 47,
                        },
                    ),
                ),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
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
                                ItemSynNodePath::MajorItem(
                                    MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath(
                                            ItemSynNodePathId {
                                                data: ItemSynNodePathData::MajorItem(
                                                    MajorItemSynNodePathData::Fugitive(
                                                        FugitiveSynNodePathData {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
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
                                                    5,
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
                ),
                path: SynNodeRegionPath::Defn(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
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
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                5,
                            ),
                            LiteralData::Float(
                                Unspecified(
                                    UnspecifiedFloatLiteral(
                                        Id {
                                            value: 128,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 1,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::FrameVarDecl {
                            regional_token_idx: RegionalTokenIdx(
                                7,
                            ),
                            ident: `i`,
                            frame_var_symbol_idx: 2,
                            current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                3,
                            ),
                        },
                        SynExprData::MethodApplicationOrCall {
                            self_argument: 2,
                            dot_regional_token_idx: RegionalTokenIdx(
                                10,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `ilen`,
                                regional_token_idx: RegionalTokenIdx(
                                    11,
                                ),
                            },
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                12,
                            ),
                            items: [],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                13,
                            ),
                        },
                        SynExprData::Binary {
                            lopd: 3,
                            opr: Comparison(
                                Less,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                8,
                            ),
                            ropd: 4,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 2,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::CurrentSynSymbol {
                            ident: `i`,
                            regional_token_idx: RegionalTokenIdx(
                                19,
                            ),
                            current_syn_symbol_idx: 2,
                            current_syn_symbol_kind: CurrentSynSymbolKind::LoopVariable(
                                3,
                            ),
                        },
                        SynExprData::IndexOrCompositionWithList {
                            owner: 6,
                            lbox_regional_token_idx: RegionalTokenIdx(
                                18,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 7,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                20,
                            ),
                        },
                        SynExprData::CurrentSynSymbol {
                            ident: `sum`,
                            regional_token_idx: RegionalTokenIdx(
                                15,
                            ),
                            current_syn_symbol_idx: 1,
                            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                pattern_symbol_idx: 1,
                            },
                        },
                        SynExprData::Field {
                            owner: 8,
                            dot_regional_token_idx: RegionalTokenIdx(
                                21,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `row_span_sum`,
                                regional_token_idx: RegionalTokenIdx(
                                    22,
                                ),
                            },
                        },
                        SynExprData::Binary {
                            lopd: 9,
                            opr: AssignClosed(
                                Add,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                16,
                            ),
                            ropd: 10,
                        },
                        SynExprData::PrincipalEntityPath {
                            path_expr_idx: 3,
                            opt_path: Some(
                                PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::CurrentSynSymbol {
                            ident: `sum`,
                            regional_token_idx: RegionalTokenIdx(
                                24,
                            ),
                            current_syn_symbol_idx: 1,
                            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                pattern_symbol_idx: 1,
                            },
                        },
                        SynExprData::Field {
                            owner: 12,
                            dot_regional_token_idx: RegionalTokenIdx(
                                27,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `row_span_sum`,
                                regional_token_idx: RegionalTokenIdx(
                                    28,
                                ),
                            },
                        },
                        SynExprData::Binary {
                            lopd: 13,
                            opr: Closed(
                                Sub,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                25,
                            ),
                            ropd: 14,
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                2..5,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `connected_components`,
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `connected_components`,
                                    regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                ),
                            ),
                        },
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `major_connected_component`,
                                    regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Eval {
                            expr_idx: 11,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                        SynStmtData::Let {
                            let_token: LetRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    1,
                                ),
                            },
                            let_variables_pattern: Ok(
                                LetPatternSynSyndicate {
                                    syn_pattern_expr_root: LetSynPatternExprRoot {
                                        syn_pattern_expr_idx: 1,
                                    },
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon_token: Ok(
                                        None,
                                    ),
                                    ty: None,
                                },
                            ),
                            assign_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                            ),
                            initial_value: 1,
                        },
                        SynStmtData::ForBetween {
                            for_token: StmtForRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    6,
                                ),
                            },
                            particulars: SynForBetweenParticulars {
                                for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                    7,
                                ),
                                for_between_loop_var_ident: `i`,
                                for_between_loop_var_expr_idx: 3,
                                range: Ok(
                                    SynForBetweenRange {
                                        initial_boundary: SynForBetweenLoopBoundary {
                                            bound_expr: None,
                                            kind: LowerClosed,
                                        },
                                        final_boundary: SynForBetweenLoopBoundary {
                                            bound_expr: Some(
                                                4,
                                            ),
                                            kind: UpperOpen,
                                        },
                                        step: Constant(
                                            1,
                                        ),
                                    },
                                ),
                            },
                            for_loop_var_symbol_idx: 2,
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            14,
                                        ),
                                    },
                                ),
                            ),
                            block: ArenaIdxRange(
                                1..2,
                            ),
                        },
                        SynStmtData::Return {
                            return_token: ReturnRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    23,
                                ),
                            },
                            result: 15,
                        },
                    ],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [
                            SynPatternExpr::Ident {
                                symbol_modifier_tokens: Some(
                                    Mut(
                                        MutRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                2,
                                            ),
                                        },
                                    ),
                                ),
                                ident_token: IdentRegionalToken {
                                    ident: `sum`,
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                            },
                        ],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [
                            Move,
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
                                `sum`,
                                1,
                            ),
                        ],
                    ],
                    pattern_symbol_modifiers: ArenaMap {
                        data: [
                            Mut,
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
                                modifier: Mut,
                                access_start: RegionalTokenIdx(
                                    4,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            29,
                                        ),
                                    ),
                                ),
                                data: CurrentSynSymbolData::LetVariable {
                                    ident: `sum`,
                                    pattern_symbol_idx: 1,
                                },
                            },
                            CurrentSynSymbol {
                                modifier: Pure,
                                access_start: RegionalTokenIdx(
                                    15,
                                ),
                                access_end: Some(
                                    RegionalTokenIdxRangeEnd(
                                        RegionalTokenIdx(
                                            23,
                                        ),
                                    ),
                                ),
                                data: CurrentSynSymbolData::LoopVariable {
                                    ident: `i`,
                                    expr_idx: 3,
                                },
                            },
                        ],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [
                        (
                            LoopVariable,
                            ArenaIdxRange(
                                2..3,
                            ),
                        ),
                    ],
                },
                syn_pattern_expr_roots: [
                    SynPatternExprRoot {
                        kind: SynPatternExprRootKind::Let,
                        syn_pattern_expr_idx: 1,
                    },
                ],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: SynExprRootKind::LetStmtInitialValue,
                        syn_expr_idx: 1,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 11,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::ReturnExpr,
                        syn_expr_idx: 15,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::BlockExpr,
                        syn_expr_idx: 16,
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
            path: Defn(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 309,
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
                                Literal(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                    Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 128,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
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
                                PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 318,
                                                    },
                                                ),
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
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 107,
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
                                        2,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 134,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            11,
                                        ),
                                    },
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: MethodFn(
                                            MethodFnFluffySignature {
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 191,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FluffyRitchieRegularParameter {
                                                    contract: Pure,
                                                    ty: FluffyTerm {
                                                        place: None,
                                                        base: Ethereal(
                                                            Application(
                                                                EtherealTermApplication(
                                                                    Id {
                                                                        value: 50,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FluffyTerm {
                                                    place: None,
                                                    base: Ethereal(
                                                        EntityPath(
                                                            TypeOntology(
                                                                TypePath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 43,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                instantiation: FluffyInstantiation {
                                                    env: MethodFn {
                                                        self_place: Leashed,
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EtherealTermSymbol(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                            Explicit(
                                                                FluffyTerm {
                                                                    place: None,
                                                                    base: Ethereal(
                                                                        EntityPath(
                                                                            TypeOntology(
                                                                                TypePath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 221,
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
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 43,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                FrameVarDecl {
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 250,
                                            },
                                        ),
                                    ),
                                    frame_var_symbol_idx: 2,
                                    current_syn_symbol_kind: LoopVariable(
                                        3,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 43,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 534,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
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
                                PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 318,
                                                    },
                                                ),
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
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 107,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 250,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    current_syn_symbol_idx: 2,
                                    current_syn_symbol_kind: LoopVariable(
                                        3,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 43,
                                                        },
                                                    ),
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
                                        6,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                7,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        signature: Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 221,
                                                                    },
                                                                ),
                                                            ),
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
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 221,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        8,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 221,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 255,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 221,
                                                },
                                            ),
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
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
                                            path: TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 383,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            instantiation: FluffyInstantiation {
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: None,
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
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
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        5,
                                    ),
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ropd: SemaExprIdx(
                                        9,
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                CurrentSynSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 534,
                                            },
                                        ),
                                    ),
                                    regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: LetVariable {
                                        pattern_symbol_idx: 1,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
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
                                PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    path: MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 319,
                                                    },
                                                ),
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
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        12,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 255,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            28,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 221,
                                                },
                                            ),
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
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
                                            path: TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 383,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            instantiation: FluffyInstantiation {
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: None,
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
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
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Binary {
                                    lopd: SemaExprIdx(
                                        11,
                                    ),
                                    opr: Closed(
                                        Sub,
                                    ),
                                    dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        25,
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
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Block {
                                    stmts: SemaStmtIdxRange(
                                        ArenaIdxRange(
                                            2..5,
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 22,
                                                        },
                                                    ),
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
                                        10,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            Trivial(
                                                TrivialFluffyCoersion {
                                                    expectee_place: Transient,
                                                },
                                            ),
                                        ),
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
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
                                    let_pattern_sema_obelisk: LetPatternSemaSyndicate {
                                        syn_pattern_root: LetSynPatternExprRoot {
                                            syn_pattern_expr_idx: 1,
                                        },
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon_token: None,
                                        ty_sema_expr_idx: None,
                                    },
                                    contract: Move,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    initial_value_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    coersion: None,
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: None,
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                ForBetween {
                                    for_token: StmtForRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    particulars: SemaForBetweenParticulars {
                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                        for_between_loop_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 250,
                                                },
                                            ),
                                        ),
                                        for_between_loop_var_expr_idx: SemaExprIdx(
                                            4,
                                        ),
                                        range: SemaForBetweenRange {
                                            initial_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: None,
                                                kind: LowerClosed,
                                            },
                                            final_boundary: SemaForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    SemaExprIdx(
                                                        3,
                                                    ),
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    for_loop_var_symbol_idx: 2,
                                    eol_colon: Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                14,
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaStmtEntry {
                            data_result: Ok(
                                Return {
                                    return_token: ReturnRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                    },
                                    result: SemaExprIdx(
                                        14,
                                    ),
                                    coersion: Some(
                                        Trivial(
                                            TrivialFluffyCoersion {
                                                expectee_place: Transient,
                                            },
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
                                                    ItemPathId(
                                                        Id {
                                                            value: 22,
                                                        },
                                                    ),
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
            sema_expr_roots: [
                (
                    16,
                    (
                        SemaExprIdx(
                            15,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternExprTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Const,
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            0,
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
                                    F32(
                                        TermF32Literal(
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
                inherited_syn_symbol_map: ArenaMap {
                    data: [],
                },
                current_syn_symbol_map: ArenaMap {
                    data: [
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Hollow(
                                        HollowTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Some(
                            SymbolType(
                                FluffyTerm {
                                    place: Some(
                                        ImmutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    2,
                                                ),
                                            ),
                                        },
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 43,
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
                        entries: [
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        1,
                                    ),
                                    hole_kind: UnspecifiedFloatType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: Some(
                                                Leashed,
                                            ),
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
                                    constraints: [
                                        CoercibleFrom {
                                            target: FluffyTerm {
                                                place: Some(
                                                    Leashed,
                                                ),
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
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
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
                        ],
                        first_unresolved_term_idx: 1,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 107,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 3,
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
                                                                value: 43,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                ImmutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 43,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 43,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: ImmutableStackOwned {
                                                            location: StackLocationIdx(
                                                                ShiftedU32(
                                                                    2,
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 107,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 43,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 221,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                MutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Leashed,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
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
                                                                value: 23,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: Some(
                                                MutableStackOwned {
                                                    location: StackLocationIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            base: Hollow(
                                                HollowTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
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
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Leashed,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
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
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
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
                                                                value: 53,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
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
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
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
                                                                value: 22,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Never,
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
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
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
                                ItemSynNodePath::MajorItem(
                                    MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath(
                                            ItemSynNodePathId {
                                                data: ItemSynNodePathData::MajorItem(
                                                    MajorItemSynNodePathData::Fugitive(
                                                        FugitiveSynNodePathData {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
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
                                            6,
                                        ),
                                        items: [],
                                        rbox_regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                    SynExprData::Prefix {
                                        opr: Tilde,
                                        opr_regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                        opd: 1,
                                    },
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 1,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 2,
                                        argument_expr_idx: 3,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `RawContour`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                    syn_expr_idx: 4,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                            syn_pattern_to_current_syn_symbol_map: [],
                        },
                    },
                ),
                path: SynNodeRegionPath::Defn(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
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
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Field {
                            owner: 1,
                            dot_regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `raw_contours`,
                                regional_token_idx: RegionalTokenIdx(
                                    3,
                                ),
                            },
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `major_connected_component`,
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Eval {
                            expr_idx: 2,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                    ],
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
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 2,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::BlockExpr,
                        syn_expr_idx: 3,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 310,
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
                                        Fugitive(
                                            FugitivePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 319,
                                                    },
                                                ),
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
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 246,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 221,
                                                },
                                            ),
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 64,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            path: TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 379,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            instantiation: FluffyInstantiation {
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: None,
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 64,
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
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 64,
                                                },
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
                                        2,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            PlaceToLeash,
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 64,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    3,
                    (
                        SemaExprIdx(
                            3,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
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
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 114,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 64,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                PlaceToLeash,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 114,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 64,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                PlaceToLeash,
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
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 114,
                        },
                    ),
                ),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
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
                                ItemSynNodePath::MajorItem(
                                    MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath(
                                            ItemSynNodePathId {
                                                data: ItemSynNodePathData::MajorItem(
                                                    MajorItemSynNodePathData::Fugitive(
                                                        FugitiveSynNodePathData {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
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
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::Prefix {
                                        opr: Tilde,
                                        opr_regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                        opd: 1,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `RawContour`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                    syn_expr_idx: 2,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                            syn_pattern_to_current_syn_symbol_map: [],
                        },
                    },
                ),
                path: SynNodeRegionPath::Defn(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
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
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Field {
                            owner: 1,
                            dot_regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `raw_contours`,
                                regional_token_idx: RegionalTokenIdx(
                                    3,
                                ),
                            },
                        },
                        SynExprData::Literal(
                            RegionalTokenIdx(
                                5,
                            ),
                            LiteralData::Integer(
                                UnspecifiedRegular(
                                    0,
                                ),
                            ),
                        ),
                        SynExprData::IndexOrCompositionWithList {
                            owner: 2,
                            lbox_regional_token_idx: RegionalTokenIdx(
                                4,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 3,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                6,
                            ),
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `major_connected_component`,
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Eval {
                            expr_idx: 4,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                    ],
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
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 4,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::BlockExpr,
                        syn_expr_idx: 5,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 311,
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
                                        Fugitive(
                                            FugitivePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 319,
                                                    },
                                                ),
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
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 47,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 246,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 221,
                                                },
                                            ),
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 64,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            path: TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 379,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            instantiation: FluffyInstantiation {
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: None,
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 64,
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
                                        5,
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
                                    place: Some(
                                        Const,
                                    ),
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
                                Index {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        2,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    index_sema_list_items: [
                                        SemaCommaListItem {
                                            sema_expr_idx: SemaExprIdx(
                                                3,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                    index_dynamic_dispatch: FluffyDynamicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Leashed,
                                            indirections: [],
                                            final_place: Leashed,
                                        },
                                        signature: Int {
                                            element_ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 224,
                                                                    },
                                                                ),
                                                            ),
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
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 224,
                                                        },
                                                    ),
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
                                            1..2,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 224,
                                                        },
                                                    ),
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
                                        4,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            PlaceToLeash,
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 224,
                                                        },
                                                    ),
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
            sema_expr_roots: [
                (
                    5,
                    (
                        SemaExprIdx(
                            5,
                        ),
                        BlockExpr,
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
                        3,
                    ),
                    Ok(
                        FluffyTerm {
                            place: None,
                            base: Ethereal(
                                Literal(
                                    USize(
                                        TermUSizeLiteral(
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
                        entries: [
                            HollowTermEntry {
                                data: Hole {
                                    hole_source: Expr(
                                        3,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 52,
                                                                },
                                                            ),
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
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 52,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: ResolvedEthereal(
                                    EntityPath(
                                        TypeOntology(
                                            TypePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 52,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
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
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 47,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 64,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Pure,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                EntityPath(
                                                    TypeOntology(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 52,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Const,
                                        ),
                                        base: Hollow(
                                            HollowTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                Trivial(
                                                    TrivialFluffyCoersion {
                                                        expectee_place: Const,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 46,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 224,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                PlaceToLeash,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 46,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 224,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                PlaceToLeash,
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
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 46,
                        },
                    ),
                ),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
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
                                ItemSynNodePath::MajorItem(
                                    MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath(
                                            ItemSynNodePathId {
                                                data: ItemSynNodePathData::MajorItem(
                                                    MajorItemSynNodePathData::Fugitive(
                                                        FugitiveSynNodePathData {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
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
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::Prefix {
                                        opr: Tilde,
                                        opr_regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                        opd: 1,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `LineSegmentSketch`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                    syn_expr_idx: 2,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                            syn_pattern_to_current_syn_symbol_map: [],
                        },
                    },
                ),
                path: SynNodeRegionPath::Defn(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
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
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Field {
                            owner: 1,
                            dot_regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `line_segment_sketch`,
                                regional_token_idx: RegionalTokenIdx(
                                    3,
                                ),
                            },
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `major_raw_contour`,
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Eval {
                            expr_idx: 2,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                    ],
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
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 2,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::BlockExpr,
                        syn_expr_idx: 3,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 312,
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
                                        Fugitive(
                                            FugitivePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 322,
                                                    },
                                                ),
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
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 46,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    owner_ty: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 46,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 210,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 224,
                                                },
                                            ),
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    EntityPath(
                                                        TypeOntology(
                                                            TypePath(
                                                                ItemPathId(
                                                                    Id {
                                                                        value: 250,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            path: TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 390,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            instantiation: FluffyInstantiation {
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: None,
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 250,
                                                        },
                                                    ),
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
                                            1..2,
                                        ),
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 250,
                                                        },
                                                    ),
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
                                        2,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            PlaceToLeash,
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        EntityPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 250,
                                                        },
                                                    ),
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
            sema_expr_roots: [
                (
                    3,
                    (
                        SemaExprIdx(
                            3,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
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
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: None,
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 46,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
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
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 250,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                PlaceToLeash,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
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
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            EntityPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 250,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                PlaceToLeash,
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
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 74,
                        },
                    ),
                ),
            ),
            self_ty: None,
        },
    },
    SemaExprRegion {
        path: SynNodeRegionPath::Defn(
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
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
                                ItemSynNodePath::MajorItem(
                                    MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath(
                                            ItemSynNodePathId {
                                                data: ItemSynNodePathData::MajorItem(
                                                    MajorItemSynNodePathData::Fugitive(
                                                        FugitiveSynNodePathData {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
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
                                            6,
                                        ),
                                        items: [],
                                        rbox_regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                    SynExprData::Prefix {
                                        opr: Tilde,
                                        opr_regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                        opd: 1,
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
                                        function_expr_idx: 2,
                                        argument_expr_idx: 3,
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
                                                    8,
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
                                    syn_expr_idx: 4,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                            syn_pattern_to_current_syn_symbol_map: [],
                        },
                    },
                ),
                path: SynNodeRegionPath::Defn(
                    ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
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
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    ),
                                ),
                            ),
                        },
                        SynExprData::Field {
                            owner: 1,
                            dot_regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: `concave_components`,
                                regional_token_idx: RegionalTokenIdx(
                                    3,
                                ),
                            },
                        },
                        SynExprData::Block {
                            stmts: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [
                        SynPrincipalEntityPathExpr::Root {
                            path_name_token: PathNameRegionalToken::Ident(
                                IdentRegionalToken {
                                    ident: `major_line_segment_sketch`,
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                },
                            ),
                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                ),
                            ),
                        },
                    ],
                },
                stmt_arena: Arena {
                    data: [
                        SynStmtData::Eval {
                            expr_idx: 2,
                            eol_semicolon: Ok(
                                None,
                            ),
                        },
                    ],
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
                        kind: SynExprRootKind::EvalExpr,
                        syn_expr_idx: 2,
                    },
                    SynExprRoot {
                        kind: SynExprRootKind::BlockExpr,
                        syn_expr_idx: 3,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
        },
        data: SemaExprRegionData {
            path: Defn(
                MajorItem(
                    Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 313,
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
                                        Fugitive(
                                            FugitivePath(
                                                ItemPathId(
                                                    Id {
                                                        value: 323,
                                                    },
                                                ),
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
                        },
                        SemaExprEntry {
                            data_result: Ok(
                                Field {
                                    owner_sema_expr_idx: SemaExprIdx(
                                        1,
                                    ),
                                    owner_ty: FluffyTerm {
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
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 367,
                                                },
                                            ),
                                        ),
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    dispatch: FluffyFieldDyanmicDispatch {
                                        indirections: FluffyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed,
                                        },
                                        ty_path: TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 250,
                                                },
                                            ),
                                        ),
                                        signature: Memoized {
                                            ty: FluffyTerm {
                                                place: None,
                                                base: Ethereal(
                                                    Application(
                                                        EtherealTermApplication(
                                                            Id {
                                                                value: 70,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            path: TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 433,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            instantiation: FluffyInstantiation {
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: None,
                                            },
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
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
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
                                                },
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
                                        2,
                                    ),
                                    outcome: Some(
                                        Coersion(
                                            PlaceToLeash,
                                        ),
                                    ),
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ),
                            ty_result: Ok(
                                FluffyTerm {
                                    place: Some(
                                        Leashed,
                                    ),
                                    base: Ethereal(
                                        Application(
                                            EtherealTermApplication(
                                                Id {
                                                    value: 70,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sema_expr_roots: [
                (
                    3,
                    (
                        SemaExprIdx(
                            3,
                        ),
                        BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: ArenaMap {
                data: [],
            },
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sema_expr_terms: [],
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
                                expectation: AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                meta: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
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
                                    resolve_progress: Intact,
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 71,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 70,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                PlaceToLeash,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FluffyTermExpectationEntry {
                                expectation: ImplicitlyConvertible(
                                    ExpectCoersion {
                                        contract: Move,
                                        ty_expected: FluffyTerm {
                                            place: None,
                                            base: Ethereal(
                                                Application(
                                                    EtherealTermApplication(
                                                        Id {
                                                            value: 71,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                meta: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FluffyTerm {
                                        place: Some(
                                            Leashed,
                                        ),
                                        base: Ethereal(
                                            Application(
                                                EtherealTermApplication(
                                                    Id {
                                                        value: 70,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    resolve_progress: Resolved(
                                        Ok(
                                            Coersion(
                                                PlaceToLeash,
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
                Application(
                    EtherealTermApplication(
                        Id {
                            value: 71,
                        },
                    ),
                ),
            ),
            self_ty: None,
        },
    },
]